use std::collections::HashMap;

use actix::prelude::*;

use super::super::supervisor_pipeline_life_time::actor_wrapper::SupervisorPipelineBridge;
use crate::logging::AppLogger;

static LOGGER: AppLogger = AppLogger::new(
    "iot_bee::adapters::actor_system::supervisor_actor_system::SystemActorSupervisor",
);

// ── SystemActorSupervisor ─────────────────────────────────────────────────────
//
// Actor de nivel superior: gestiona un PipelineSupervisor por cada pipeline_id.
// Cada PipelineSupervisor gestiona N réplicas de ese pipeline.
//
// - supervisors: HashMap<pipeline_id, SupervisorPipelineBridge>
//
// Helpers pub(super) para que handlers.rs pueda usarlos sin duplicar lógica:
//   get_bridge(id)          → clona el bridge si existe
//   insert_pipeline(id, b)  → registra un bridge nuevo
//   remove_pipeline(id)     → extrae y devuelve el bridge
//   list_pipeline_ids()     → copia de las claves

pub struct SystemActorSupervisor {
    supervisors: HashMap<u32, SupervisorPipelineBridge>,
}

impl SystemActorSupervisor {
    pub fn new() -> Self {
        Self {
            supervisors: HashMap::new(),
        }
    }

    /// Clona el bridge del pipeline dado, si existe.
    pub(super) fn get_bridge(&self, pipeline_id: u32) -> Option<SupervisorPipelineBridge> {
        self.supervisors.get(&pipeline_id).cloned()
    }

    /// Registra un bridge nuevo para el pipeline dado.
    pub(super) fn insert_pipeline(&mut self, pipeline_id: u32, bridge: SupervisorPipelineBridge) {
        self.supervisors.insert(pipeline_id, bridge);
    }

    /// Elimina y devuelve el bridge del pipeline dado.
    pub(super) fn remove_pipeline(
        &mut self,
        pipeline_id: u32,
    ) -> Option<SupervisorPipelineBridge> {
        
        self.supervisors.remove(&pipeline_id)
    }

    /// Devuelve una copia de los pipeline_ids registrados.
    pub(super) fn list_pipeline_ids(&self) -> Vec<u32> {
        self.supervisors.keys().copied().collect()
    }
}

impl Actor for SystemActorSupervisor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        LOGGER.info("SystemActorSupervisor started. Ready to manage pipeline supervisors.");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        LOGGER.info(&format!(
            "SystemActorSupervisor stopped with {} pipeline supervisor(s) registered.",
            self.supervisors.len()
        ));
    }
}

impl Supervised for SystemActorSupervisor {
    fn restarting(&mut self, _ctx: &mut Self::Context) {
        LOGGER.warn("SystemActorSupervisor is restarting.");
    }
}




// brigde wrapper para enviar mensajes a SystemActorSupervisor sin exponer su ActorAddr ni lógica de routing a handlers.rs
// use crate::domain::inbound::pipeline_lifecycle::PipelineLifecycle;
// use crate::domain::value_objects::pipelines_values::{PipelineStatus,DataStoreId};
// use async_trait ::async_trait;

// use super::messages::{
//     CreatePipelineMessage, DeletePipelineMessage, ListPipelinesMessage,
//     SystemAddReplicaMessage, SystemRemoveReplicaMessage,
// };


// pub struct PipelineActorSupervisorSystemBridge {
//     supervisor_addr: Addr<SystemActorSupervisor>,
// }

// #[async_trait]
// impl PipelineLifecycle for PipelineActorSupervisorSystemBridge {
//     // Implementación de los métodos de PipelineLifecycle aquí
//     async fn get_status_by_id(
//         &self,
//         pipeline_id: &DataStoreId,
//     ) -> Result<PipelineStatus, IoTBeeError> {
//        let result = self.supervisor_addr
//             .send(GetPipelineStatusMessage::new(pipeline_id.clone()))
//             .await
//             .map_err(mailbox_err)?;
//        Ok(result)
//     }    
// }