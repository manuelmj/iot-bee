use actix::prelude::*;
use crate::domain::outbound::data_processor_actions::DataProcessorActions;
use crate::domain::outbound::data_external_store::DataExternalStore;
use crate::domain::outbound::data_source::DataSource;
use std::sync::Arc;
use super::pipeline_abstraction::ReplicaRegistry;
use crate::logging::AppLogger;

static LOGGER: AppLogger = AppLogger::new(
    "iot_bee::adapters::actor_system::supervisor_pipeline_life_time::PipelineSupervisor",
);

// ── PipelineSupervisor ────────────────────────────────────────────────────────
//
// Actor que gestiona las réplicas de UN pipeline concreto.
// Cada réplica es un PipelineAbstractionController (consumer + processor + store).
//
// El campo pipeline_id identifica de qué pipeline es supervisor.
// La lógica de routing entre pipelines vive en SystemActorSupervisor (nivel superior).
//
// Handlers síncronos (add, remove, count):
//   type Result = Result<_, IoTBeeError>  → el mailbox continúa
//
// Handlers asíncronos (stop_all, restart_all, status_all):
//   type Result = ResponseFuture<_>       → lanza Box::pin sin bloquear el mailbox.
//   Los Arc de las réplicas se clonan en la parte síncrona (all_arcs()) antes del
//   await, garantizando que el RwLockGuard se libere antes de cualquier suspensión.

type DataSourceThreadSafe = Arc<dyn DataSource + Send + Sync + 'static>;
type DataProcessorThreadSafe = Arc<dyn DataProcessorActions + Send + Sync + 'static>;
type DataExternalStoreThreadSafe = Arc<dyn DataExternalStore + Send + Sync + 'static>;


pub struct PipelineRuntimeConfig{
    pub pipeline_replica_count: usize,
    pub pipeline_name: String, 
}
pub struct PipelineSupervisor {
    pipeline_id: u32,
    replica_registry: ReplicaRegistry,
    pipeline_configuration: PipelineRuntimeConfig, // por ahora solo un string, pero podría ser una struct con más info
    data_source: DataSourceThreadSafe,
    data_processor: DataProcessorThreadSafe,
    data_store: DataExternalStoreThreadSafe,
}


impl PipelineSupervisor {
    pub fn new(pipeline_id: u32, pipeline_configuration: PipelineRuntimeConfig, data_source: DataSourceThreadSafe, data_processor: DataProcessorThreadSafe, data_store: DataExternalStoreThreadSafe) -> Self {
        Self {
            pipeline_id,
            replica_registry: ReplicaRegistry::new(),
            pipeline_configuration,
            data_source,
            data_processor,
            data_store,
        }
    }
    pub fn pipeline_id(&self) -> u32 {
        self.pipeline_id
    }
    pub fn pipeline_configuration(&self) -> &PipelineRuntimeConfig {
        &self.pipeline_configuration
    }
    pub fn replica_registry(&mut self) -> &mut ReplicaRegistry {
        &mut self.replica_registry
    }

    pub fn data_source(&self) -> DataSourceThreadSafe {
        self.data_source.clone()
    }

    pub fn data_processor(&self) -> DataProcessorThreadSafe {
        self.data_processor.clone()
    }

    pub fn data_store(&self) -> DataExternalStoreThreadSafe {
        self.data_store.clone()
    }

}

impl Actor for PipelineSupervisor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        LOGGER.info(&format!(
            "PipelineSupervisor started for pipeline_id={}.",
            self.pipeline_id
        ));
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        match self.replica_registry.replica_count() {
            Ok(n) if n > 0 => {
                LOGGER.warn(&format!(
                    "PipelineSupervisor for pipeline_id={} stopped with {} replica(s) still registered.",
                    self.pipeline_id, n
                ));
            }
            _ => LOGGER.info(&format!(
                "PipelineSupervisor for pipeline_id={} stopped.",
                self.pipeline_id
            )),
        }
    }
}

impl Supervised for PipelineSupervisor {
    fn restarting(&mut self, _ctx: &mut Self::Context) {
        LOGGER.warn(&format!(
            "PipelineSupervisor for pipeline_id={} is restarting.",
            self.pipeline_id
        ));
    }
}
