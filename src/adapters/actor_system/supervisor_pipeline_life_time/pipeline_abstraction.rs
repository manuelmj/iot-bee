
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::super::pipeline_actor_module::general_ports::SendActionToActor;
use super::super::pipeline_actor_module::general_messages::SendActorActionMessageResult;
use crate::domain::error::{IoTBeeError, PipelineLifecycleError};

// Alias público para que messges.rs y handlers.rs puedan referenciarlo.
pub type TripleResult = (
    SendActorActionMessageResult,
    SendActorActionMessageResult,
    SendActorActionMessageResult,
);

// ── PipelineAbstractionController ─────────────────────────────────────────────
//
// Agrupa los tres bridges (consumer, processor, store) de un pipeline activo.
// Las acciones de ciclo de vida se delegan a los tres en orden.

pub struct PipelineAbstractionController<T, U, V>
where
    T: SendActionToActor,
    U: SendActionToActor,
    V: SendActionToActor,
{
    consumer: T,
    processor: U,
    store: V,
}

impl<T, U, V> PipelineAbstractionController<T, U, V>
where
    T: SendActionToActor,
    U: SendActionToActor,
    V: SendActionToActor,
{
    pub fn new(consumer: T, processor: U, store: V) -> Self {
        Self { consumer, processor, store }
    }

    /// Detiene los tres actores: consumer → processor → store.
    pub async fn stop(&self) -> TripleResult {
        (
            self.consumer.send_stop_actor().await,
            self.processor.send_stop_actor().await,
            self.store.send_stop_actor().await,
        )
    }

    /// Reinicia los tres actores.
    pub async fn restart(&self) -> TripleResult {
        (
            self.consumer.send_restart_actor().await,
            self.processor.send_restart_actor().await,
            self.store.send_restart_actor().await,
        )
    }

    /// Consulta el estado de los tres actores.
    pub async fn status(&self) -> TripleResult {
        (
            self.consumer.get_actor_status().await,
            self.processor.get_actor_status().await,
            self.store.get_actor_status().await,
        )
    }
}

// ── PipelineRegistry ──────────────────────────────────────────────────────────
//
// HashMap<id, Arc<Controller>> protegido con RwLock.
// Se almacena Arc para poder clonar la referencia antes de cualquier .await,
// garantizando que el RwLockGuard nunca se sostenga a través de un punto de
// suspensión asíncrona (lo que causaría un deadlock en tokio).

pub struct PipelineRegistry<T, U, V>
where
    T: SendActionToActor,
    U: SendActionToActor,
    V: SendActionToActor,
{
    pipelines: RwLock<HashMap<u32, Arc<PipelineAbstractionController<T, U, V>>>>,
}

impl<T, U, V> PipelineRegistry<T, U, V>
where
    T: SendActionToActor,
    U: SendActionToActor,
    V: SendActionToActor,
{
    pub fn new() -> Self {
        let pipelines: HashMap<u32, Arc<PipelineAbstractionController<T, U, V>>> = HashMap::new();
        Self { pipelines: RwLock::new(pipelines) }
    }

    // ── Helpers privados de acceso al RwLock ─────────────────────────────────
    //
    // Encapsulan el tratamiento de lock envenenado en un único punto:
    // si un hilo entró en pánico sosteniendo el lock, el error se propaga
    // limpiamente en lugar de hundir el thread actual con expect/unwrap.

    fn read_lock(
        &self,
    ) -> Result<
        std::sync::RwLockReadGuard<'_, HashMap<u32, Arc<PipelineAbstractionController<T, U, V>>>>,
        IoTBeeError,
    > {
        self.pipelines.read().map_err(|_| {
            PipelineLifecycleError::OperationFailed {
                reason: "PipelineRegistry: el read-lock está envenenado \
                         (un hilo anterior entró en pánico sosteniéndolo)"
                    .to_string(),
            }
            .into()
        })
    }

    fn write_lock(
        &self,
    ) -> Result<
        std::sync::RwLockWriteGuard<'_, HashMap<u32, Arc<PipelineAbstractionController<T, U, V>>>>,
        IoTBeeError,
    > {
        self.pipelines.write().map_err(|_| {
            PipelineLifecycleError::OperationFailed {
                reason: "PipelineRegistry: el write-lock está envenenado \
                         (un hilo anterior entró en pánico sosteniéndolo)"
                    .to_string(),
            }
            .into()
        })
    }

    /// Obtiene un Arc al controller, liberando el lock inmediatamente.
    /// `pub(super)` para que los handlers del mismo módulo puedan usarlo
    /// dentro de sus `Box::pin(async move {})` sin sostener el guard.
    pub(super) fn get_controller(
        &self,
        id: u32,
    ) -> Result<Arc<PipelineAbstractionController<T, U, V>>, IoTBeeError> {
        self.read_lock()?
            .get(&id)
            .cloned()
            .ok_or_else(|| {
                PipelineLifecycleError::NotFound { pipeline_id: id.to_string() }.into()
            })
    }

    // ── API pública ──────────────────────────────────────────────────────────

    /// Agrega un pipeline al registro. Error si el id ya existe.
    pub fn add(
        &self,
        id: u32,
        controller: PipelineAbstractionController<T, U, V>,
    ) -> Result<(), IoTBeeError> {
        let mut map = self.write_lock()?;
        if map.contains_key(&id) {
            return Err(PipelineLifecycleError::AlreadyRunning {
                pipeline_id: id.to_string(),
            }
            .into());
        }
        map.insert(id, Arc::new(controller));
        Ok(())
    }

    /// Elimina el pipeline del registro y devuelve su controller.
    /// Error si el id no existe.
    pub fn remove(
        &self,
        id: u32,
    ) -> Result<Arc<PipelineAbstractionController<T, U, V>>, IoTBeeError> {
        self.write_lock()?
            .remove(&id)
            .ok_or_else(|| {
                PipelineLifecycleError::NotFound { pipeline_id: id.to_string() }.into()
            })
    }

    /// Lista los ids de todos los pipelines registrados.
    pub fn list_ids(&self) -> Result<Vec<u32>, IoTBeeError> {
        Ok(self.read_lock()?.keys().copied().collect())
    }

    /// Acceso de solo lectura sincrónico a un pipeline mediante closure.
    /// El lock se libera al salir del closure; no usar para operaciones async.
    pub fn with<F, R>(&self, id: u32, f: F) -> Result<R, IoTBeeError>
    where
        F: FnOnce(&PipelineAbstractionController<T, U, V>) -> R,
    {
        let map = self.read_lock()?;
        map.get(&id)
            .map(|c| f(c))
            .ok_or_else(|| PipelineLifecycleError::NotFound { pipeline_id: id.to_string() }.into())
    }

    // ── Acciones de ciclo de vida ────────────────────────────────────────────
    //
    // Patrón: get_controller() clona el Arc y libera el lock ANTES del .await.
    // Sin este patrón el RwLockGuard quedaría suspendido en el executor de
    // tokio, bloqueando cualquier otro intento de tomar el lock en ese hilo.

    pub async fn stop(&self, id: u32) -> Result<TripleResult, IoTBeeError> {
        let controller = self.get_controller(id)?; // lock tomado y liberado
        Ok(controller.stop().await)                // await sin lock
    }

    pub async fn restart(&self, id: u32) -> Result<TripleResult, IoTBeeError> {
        let controller = self.get_controller(id)?;
        Ok(controller.restart().await)
    }

    pub async fn status(&self, id: u32) -> Result<TripleResult, IoTBeeError> {
        let controller = self.get_controller(id)?;
        Ok(controller.status().await)
    }
}



