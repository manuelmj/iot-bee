use actix::prelude::*;

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

pub struct PipelineSupervisor {
    pub pipeline_id: u32,
    pub(super) replica_registry: ReplicaRegistry,
}

impl PipelineSupervisor {
    pub fn new(pipeline_id: u32) -> Self {
        Self {
            pipeline_id,
            replica_registry: ReplicaRegistry::new(),
        }
    }
}

impl Actor for PipelineSupervisor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        LOGGER.info(&format!(
            "PipelineSupervisor started for pipeline_id={}.",
            self.pipeline_id
        ));
        // una vez iniciado el actor entonces usar las configuraciones para iniciar las replicas.
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
