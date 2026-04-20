use actix::prelude::*;
use async_trait::async_trait;

use super::messages::ProcessDataMessage;
use crate::adapters::actor_system::pipeline_actor_module::general_ports::SendDataToProcessor;
use crate::adapters::actor_system::pipeline_actor_module::general_ports::SendDataToStore;
use crate::domain::entities::data_consumer_types::DataConsumerRawType;
use crate::domain::error::{IoTBeeError, PipelineLifecycleError};
use crate::logging::AppLogger;
use std::sync::Arc;

static LOGGER: AppLogger = AppLogger::new(
    "iot_bee::adapters::actor_system::pipeline_actor_module::processor_actor::DataProcessorActor",
);

// ── Actor ────────────────────────────────────────────────────────────────────

pub struct DataProcessorActor<T: SendDataToStore + Send + Sync + 'static> {
    data_store: Arc<T>,
}
impl<T: SendDataToStore + Send + Sync + 'static> DataProcessorActor<T> {
    pub fn new(data_store: Arc<T>) -> Self {
        Self { data_store }
    }
    pub fn data_store(&self) -> Arc<T> {
        Arc::clone(&self.data_store)
    }
}

impl<T: SendDataToStore + Send + Sync + 'static> Actor for DataProcessorActor<T> {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        LOGGER.info("DataProcessorActor started.");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        LOGGER.info("DataProcessorActor stopped.");
    }
}

impl<T: SendDataToStore + Send + Sync + 'static> Supervised for DataProcessorActor<T> {
    fn restarting(&mut self, _ctx: &mut Self::Context) {
        LOGGER.warn("DataProcessorActor is restarting.");
    }
}

// ── Bridge ───────────────────────────────────────────────────────────────────
// Adapta Addr<DataProcessorActor> al trait SendDataToProcessor.
// El consumer nunca conoce al actor; solo conoce el trait.
//──────────────────────────────────────────────────────────────────────────────

pub struct ProcessorActorBridge<T: SendDataToStore + Send + Sync + 'static> {
    addr: Addr<DataProcessorActor<T>>,
}
//este es el que debo inyectar en el consumer actor para que pueda enviarle datos al processor actor sin conocerlo directamente.
impl<T> ProcessorActorBridge<T> 
where T: SendDataToStore + Send + Sync + 'static 
{
    pub fn new(addr: Addr<DataProcessorActor<T>>) -> Self {
        Self { addr }
    }
}

#[async_trait]
impl<T> SendDataToProcessor for ProcessorActorBridge<T>
where T: SendDataToStore + Send + Sync + 'static {  
    async fn send(&self, data: &DataConsumerRawType) -> Result<(), IoTBeeError> {
        self.addr
            .send(ProcessDataMessage::new(data.clone()))
            .await
            .map_err(|e| PipelineLifecycleError::InternalCommunication {
                reason: format!("Failed to send message to processor actor: {}", e),
            })?
    }
}
use super::super::general_messages::{SendActorActionMessageResult, SendActorActionMessage};
use super::super::general_ports::SendActionToActor;

#[async_trait]
impl <T> SendActionToActor for ProcessorActorBridge<T>
where T: SendDataToStore + Send + Sync + 'static 
{
    

    async fn send_stop_actor(&self) -> SendActorActionMessageResult {
        self.addr
            .send(SendActorActionMessage::stop())
            .await
            .map_err(|e| PipelineLifecycleError::InternalCommunication {
                reason: format!("Failed to send stop message to processor actor: {}", e),
            })?
    }

    async fn send_restart_actor(&self) -> SendActorActionMessageResult {
        self.addr
            .send(SendActorActionMessage::restart())
            .await
            .map_err(|e| PipelineLifecycleError::InternalCommunication {
                reason: format!("Failed to send restart message to processor actor: {}", e),
            })?
    }

    async fn get_actor_status(&self) -> SendActorActionMessageResult {
        self.addr
            .send(SendActorActionMessage::status())
            .await
            .map_err(|e| PipelineLifecycleError::InternalCommunication {
                reason: format!("Failed to send get status message to processor actor: {}", e),
            })?
        }
}

//────────────────────────────────────────────────────────────────────────────────────────────────────────
