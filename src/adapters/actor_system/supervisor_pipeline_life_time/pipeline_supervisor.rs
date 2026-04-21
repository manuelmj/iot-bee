use actix::prelude::*;

use super::pipeline_abstraction::PipelineRegistry;
use crate::logging::AppLogger;

static LOGGER: AppLogger = AppLogger::new(
    "iot_bee::adapters::actor_system::supervisor_pipeline_life_time::PipelineSupervisor",
);

// ── PipelineSupervisor ────────────────────────────────────────────────────────
//
// Actor que gestiona el ciclo de vida de los pipelines hijos.
// Usa PipelineRegistry como estado interno para llevar el registro.
//
// Handlers síncronos (add, remove, list):
//   type Result = Result<_, IoTBeeError>  → el mailbox continúa
//
// Handlers asíncronos (stop, restart, status):
//   type Result = ResponseFuture<_>       → lanza Box::pin sin bloquear
//   el mailbox, maximizando throughput. El Arc del controller se clona
//   en la parte síncrona del handler (antes del await) para liberar
//   el RwLock antes de suspender.

pub struct PipelineSupervisor {
    pub(super) child_registry: PipelineRegistry,
}

impl PipelineSupervisor {
    pub fn new() -> Self {
        Self {
            child_registry: PipelineRegistry::new(),
        }
    }
}

impl Actor for PipelineSupervisor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        LOGGER.info("PipelineSupervisor started. Ready to manage pipelines.");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        match self.child_registry.list_ids() {
            Ok(ids) if !ids.is_empty() => {
                LOGGER.warn(&format!(
                    "PipelineSupervisor stopped with {} pipeline(s) still registered: {:?}",
                    ids.len(),
                    ids
                ));
            }
            _ => LOGGER.info("PipelineSupervisor stopped."),
        }
    }
}

impl Supervised for PipelineSupervisor {
    fn restarting(&mut self, _ctx: &mut Self::Context) {
        LOGGER.warn("PipelineSupervisor is restarting. All child pipelines will be stopped and restarted.");
    }
}
