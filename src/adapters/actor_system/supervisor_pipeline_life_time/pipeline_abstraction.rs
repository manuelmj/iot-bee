use std::sync::{Arc};

use super::super::pipeline_actor_module::general_messages::SendActorActionMessageResult;
use super::super::pipeline_actor_module::general_ports::SendActionToActor;
use crate::domain::error::{IoTBeeError, PipelineLifecycleError};
use crate::adapters::actor_system::pipeline_actor_module::general_messages::{ResponseActorActionMessage, ActorStatus};
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
    consumer: Option<Arc<dyn SendActionToActor + Send + Sync>>,
    processor: Option<Arc<dyn SendActionToActor + Send + Sync>>,
    store: Option<Arc<dyn SendActionToActor + Send + Sync>>,
}

impl PipelineAbstractionController {
    pub fn new(
        consumer: Arc<dyn SendActionToActor + Send + Sync>,
        processor: Arc<dyn SendActionToActor + Send + Sync>,
        store: Arc<dyn SendActionToActor + Send + Sync>,
    ) -> Self {
        Self {
            consumer: Some(consumer),
            processor: Some(processor),
            store: Some(store),
        }
    }

    pub async fn stop(&mut self) -> Result<(), IoTBeeError> {
        let mut failures = Vec::new();
        
        if let Some(consumer) = self.consumer.take() {
            if let Err(e) = consumer.send_stop_actor().await {
                failures.push(("consumer", e.to_string()));
                self.consumer = Some(consumer); // reponer el consumer para intentar detener los otros actores
            }
        }
        
        if let Some(processor) = self.processor.take() {
            if let Err(e) = processor.send_stop_actor().await {
                failures.push(("processor", e.to_string()));
                self.processor = Some(processor); // reponer el processor para intentar detener los otros actores

            }
        }
        
        if let Some(store) = self.store.take() {
            if let Err(e) = store.send_stop_actor().await {
                failures.push(("store", e.to_string()));
                self.store = Some(store); // reponer el store para intentar detener los otros actores
            }
        }
        
        if failures.is_empty() {
            Ok(())
        } else {
            Err(PipelineLifecycleError::OperationFailed {
                reason: format!("Failed when stopping actors: {:?}", failures),
            }.into())
        }
    }

    pub fn pipeline_stopped(&self) -> bool {
        self.consumer.is_none() && self.processor.is_none() && self.store.is_none()

    }

    pub async fn restart(&self) -> Result<(), IoTBeeError> {
        Ok(())
    }

    pub async fn status(&self) -> Result<(), IoTBeeError> {
        Ok(())
    }
}

// ── ReplicaRegistry ───────────────────────────────────────────────────────────
//
// Vec<Arc<Controller>> protegido con RwLock.
// Cada elemento es una réplica del pipeline; el índice posicional la identifica.
// Se almacena Arc para poder clonar referencias antes de cualquier .await,
// garantizando que el RwLockGuard nunca se sostenga a través de un punto de
// suspensión asíncrona (lo que causaría un deadlock en tokio).
use tokio::sync::{RwLock,RwLockReadGuard,RwLockWriteGuard}; 
pub struct ReplicaRegistry {
    replicas: RwLock<Vec<Arc<PipelineAbstractionController>>>,
}

impl ReplicaRegistry {
    pub fn new() -> Self {
        Self {
            replicas: RwLock::new(Vec::new()),
        }
    }

    async fn read_lock(
        &self,
    ) -> RwLockReadGuard<'_, Vec<Arc<PipelineAbstractionController>>>
    {
        self.replicas.read().await 
    }

    async fn write_lock(
        &self,
    ) -> RwLockWriteGuard<'_, Vec<Arc<PipelineAbstractionController>>>
    {
        self.replicas.write().await
    }

    /// Clona todos los Arc liberando el lock inmediatamente.
    /// Usado por los handlers antes del .await para no sostener el guard.
    pub async fn all_arcs(&self) -> Vec<Arc<PipelineAbstractionController>> {
        self.read_lock().await.iter().cloned().collect()
    }

    /// Añade una réplica. Devuelve el número total de réplicas tras la inserción.
    pub async fn add_replica(
        &self,
        controller: PipelineAbstractionController,
    ) -> usize {
        let mut replicas = self.write_lock().await;
        replicas.push(Arc::new(controller));
        replicas.len()
    }

    pub async fn get_last_replica(&self) -> Result<Arc<PipelineAbstractionController>, IoTBeeError> {
        self.read_lock().await
            .last()
            .cloned()
            .ok_or_else(|| PipelineLifecycleError::OperationFailed {
                reason: "No hay réplicas disponibles".to_string(),
            }
            .into())
    }

    /// Elimina la última réplica (escala hacia abajo). Error si no hay réplicas.
    pub async fn remove_last_replica(&self) -> Result<(), IoTBeeError> {
        let mut replicas = self.write_lock().await;
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
    pub async fn replica_count(&self) -> usize {
        self.read_lock().await.len()
    }
}
