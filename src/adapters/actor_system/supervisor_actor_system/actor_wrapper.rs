// brigde wrapper para enviar mensajes a SystemActorSupervisor sin exponer su ActorAddr ni lógica de routing a handlers.rs
use crate::domain::inbound::pipeline_lifecycle::PipelineLifecycle;
use crate::domain::value_objects::pipelines_values::DataStoreId;
use async_trait::async_trait;

use super::messages::{
    CreatePipelineMessage,
    // DeletePipelineMessage, ListPipelinesMessage, SystemAddReplicaMessage,
    // SystemRemoveReplicaMessage,
};

use crate::domain::error::{IoTBeeError, PipelineLifecycleError};
use actix::prelude::*;

use crate::domain::entities::pipeline_data::PipelineConfiguration;
use crate::domain::outbound::{
    data_external_store::DataExternalStore, data_processor_actions::DataProcessorActions,
    data_source::DataSource,
};
use std::sync::Arc;

use super::system_supervisor::SystemActorSupervisor;

pub struct PipelineActorSupervisorSystemBridge {
    pub supervisor_addr: Addr<SystemActorSupervisor>,
}

impl PipelineActorSupervisorSystemBridge {
    pub fn new() -> Self {
        // let supervisor_addr = SystemActorSupervisor::new().start();
        let system_supervisor = SystemActorSupervisor::new();
        let supervisor_addr = Supervisor::start(move |_ctx| system_supervisor);
        Self { supervisor_addr }
    }
}

//Esto espara poder llamar al tipo en los casos de uso
#[async_trait]
impl PipelineLifecycle for PipelineActorSupervisorSystemBridge {
    // Implementación de los métodos de PipelineLifecycle aquí

    // para iniciar el pipeline debo enviarle la inyeccion de las dependencias necesarias para el pipeline
    async fn start(
        &self,
        pipeline_id: &DataStoreId,
        pipeline_config: PipelineConfiguration,
        data_source: Arc<dyn DataSource + Send + Sync>,
        data_processor: Arc<dyn DataProcessorActions + Send + Sync>,
        data_store: Arc<dyn DataExternalStore + Send + Sync>,
    ) -> Result<(), IoTBeeError> {
        let message_to_send = CreatePipelineMessage::new(
            pipeline_id.id(),
            pipeline_config,
            data_source,
            data_processor,
            data_store,
        );

        self.supervisor_addr
            .send(message_to_send)
            .await
            .map_err(mailbox_err)?
    }
}

fn mailbox_err(e: MailboxError) -> IoTBeeError {
    PipelineLifecycleError::InternalCommunication {
        reason: format!("Fallo de comunicación con PipelineSupervisor: {}", e),
    }
    .into()
}
