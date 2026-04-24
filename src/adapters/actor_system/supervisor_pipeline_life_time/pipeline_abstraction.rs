use std::sync::{Arc, RwLock};

use super::super::pipeline_actor_module::general_messages::SendActorActionMessageResult;
use super::super::pipeline_actor_module::general_ports::SendActionToActor;
use crate::domain::error::{IoTBeeError, PipelineLifecycleError};

/// Resultado de una operación de ciclo de vida sobre un trío (consumer, processor, store).
pub type TripleResult = (
    SendActorActionMessageResult,
    SendActorActionMessageResult,
    SendActorActionMessageResult,
);

/// Resultado de una operación de ciclo de vida sobre todas las réplicas activas.
pub type AllReplicasResult = Vec<TripleResult>;

// ── PipelineAbstractionController ─────────────────────────────────────────────
//
// Representa un trío de actores (consumer, processor, store) para una réplica.
// Las acciones de ciclo de vida se delegan a los tres en orden.

pub struct PipelineAbstractionController {
    consumer: Box<dyn SendActionToActor>,
    processor: Box<dyn SendActionToActor>,
    store: Box<dyn SendActionToActor>,
}

impl PipelineAbstractionController {
    pub fn new(
        consumer: Box<dyn SendActionToActor>,
        processor: Box<dyn SendActionToActor>,
        store: Box<dyn SendActionToActor>,
    ) -> Self {
        Self { consumer, processor, store }
    }

    pub async fn stop(&self) -> TripleResult {
        (
            self.consumer.send_stop_actor().await,
            self.processor.send_stop_actor().await,
            self.store.send_stop_actor().await,
        )
    }

    pub async fn restart(&self) -> TripleResult {
        (
            self.consumer.send_restart_actor().await,
            self.processor.send_restart_actor().await,
            self.store.send_restart_actor().await,
        )
    }

    pub async fn status(&self) -> TripleResult {
        (
            self.consumer.get_actor_status().await,
            self.processor.get_actor_status().await,
            self.store.get_actor_status().await,
        )
    }
}

// ── ReplicaRegistry ───────────────────────────────────────────────────────────
//
// Vec<Arc<Controller>> protegido con RwLock.
// Cada elemento es una réplica del pipeline; el índice posicional la identifica.
// Se almacena Arc para poder clonar referencias antes de cualquier .await,
// garantizando que el RwLockGuard nunca se sostenga a través de un punto de
// suspensión asíncrona (lo que causaría un deadlock en tokio).

pub struct ReplicaRegistry {
    replicas: RwLock<Vec<Arc<PipelineAbstractionController>>>,
}

impl ReplicaRegistry {
    pub fn new() -> Self {
        Self { replicas: RwLock::new(Vec::new()) }
    }

    fn read_lock(
        &self,
    ) -> Result<std::sync::RwLockReadGuard<'_, Vec<Arc<PipelineAbstractionController>>>, IoTBeeError>
    {
        self.replicas.read().map_err(|_| {
            PipelineLifecycleError::OperationFailed {
                reason: "ReplicaRegistry: el read-lock está envenenado \
                         (un hilo anterior entró en pánico sosteniéndolo)"
                    .to_string(),
            }
            .into()
        })
    }

    fn write_lock(
        &self,
    ) -> Result<std::sync::RwLockWriteGuard<'_, Vec<Arc<PipelineAbstractionController>>>, IoTBeeError>
    {
        self.replicas.write().map_err(|_| {
            PipelineLifecycleError::OperationFailed {
                reason: "ReplicaRegistry: el write-lock está envenenado \
                         (un hilo anterior entró en pánico sosteniéndolo)"
                    .to_string(),
            }
            .into()
        })
    }

    /// Clona todos los Arc liberando el lock inmediatamente.
    /// Usado por los handlers antes del .await para no sostener el guard.
    pub(super) fn all_arcs(
        &self,
    ) -> Result<Vec<Arc<PipelineAbstractionController>>, IoTBeeError> {
        Ok(self.read_lock()?.iter().cloned().collect())
    }

    /// Añade una réplica. Devuelve el número total de réplicas tras la inserción.
    pub fn add_replica(
        &self,
        controller: PipelineAbstractionController,
    ) -> Result<usize, IoTBeeError> {
        let mut replicas = self.write_lock()?;
        replicas.push(Arc::new(controller));
        Ok(replicas.len())
    }

    /// Elimina la última réplica (escala hacia abajo). Error si no hay réplicas.
    pub fn remove_last_replica(&self) -> Result<(), IoTBeeError> {
        let mut replicas = self.write_lock()?;
        if replicas.is_empty() {
            return Err(PipelineLifecycleError::OperationFailed {
                reason: "No hay réplicas para eliminar".to_string(),
            }
            .into());
        }
        replicas.pop();
        Ok(())
    }

    /// Devuelve el número actual de réplicas.
    pub fn replica_count(&self) -> Result<usize, IoTBeeError> {
        Ok(self.read_lock()?.len())
    }
}
