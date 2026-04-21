use actix::prelude::*;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

use crate::adapters::actor_system::pipeline_actor_module::consumer_actor::messages::{
    ConsumerActorActionMessage, ConsumerActorState,
};
use crate::adapters::actor_system::pipeline_actor_module::general_ports::SendDataToProcessor;
use crate::domain::entities::data_consumer_types::DataConsumerRawType;
use crate::domain::outbound::data_source::DataSource;

use crate::logging::AppLogger;
static LOGGER: AppLogger = AppLogger::new(
    "iot_bee::adapters::actor_system::pipeline_actor_module::consumer_actor::DataConsumerActor",
);

pub struct DataConsumerActor<
    T: DataSource + Send + Sync + 'static,
    U: SendDataToProcessor + Send + Sync + 'static,
> {
    pub data_source: Arc<T>,
    pub data_processor: Arc<U>,
    state: ConsumerActorState,
    sender: Option<Sender<DataConsumerRawType>>,
}

impl<T, U> DataConsumerActor<T, U>
where
    T: DataSource + Send + Sync + 'static,
    U: SendDataToProcessor + Send + Sync + 'static,
{
    pub fn new(data_source: Arc<T>, data_processor: Arc<U>) -> Self {
        DataConsumerActor {
            data_source,
            data_processor,
            state: ConsumerActorState::Idle,
            sender: None,
        }
    }
    pub fn data_source(&self) -> Arc<T> {
        Arc::clone(&self.data_source)
    }
    pub fn data_processor(&self) -> Arc<U> {
        Arc::clone(&self.data_processor)
    }
    pub fn sender(&self) -> Option<Sender<DataConsumerRawType>> {
        self.sender.clone()
    }
    pub fn state(&self) -> ConsumerActorState {
        self.state.clone() 
    }

    pub fn set_state(&mut self, state: ConsumerActorState) {
        self.state = state;
    }

    pub fn set_sender(&mut self, sender: Option<Sender<DataConsumerRawType>>) {
        self.sender = sender;
    }
}

impl<T, U> Actor for DataConsumerActor<T, U>
where
    T: DataSource + Send + Sync + 'static,
    U: SendDataToProcessor + Send + Sync + 'static,
{
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        LOGGER.info("DataConsumerActor started, sending StartConsuming message to self...");
        ctx.address()
            .do_send(ConsumerActorActionMessage::start_consuming());
        LOGGER.info("DataConsumerActor started, initiating data consumption...");
    }
    fn stopped(&mut self, _ctx: &mut Self::Context) {
        LOGGER.info("DataConsumerActor stopped.");
    }
    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        LOGGER.info("DataConsumerActor stopping, sending StopConsuming message to self...");
        ctx.address()
            .do_send(ConsumerActorActionMessage::stop_consuming());
        LOGGER.info("DataConsumerActor stopping, initiating shutdown of data consumption...");
        Running::Stop
    }
}

impl<T, U> Supervised for DataConsumerActor<T, U>
where
    T: DataSource + Send + Sync + 'static,
    U: SendDataToProcessor + Send + Sync + 'static,
{
    fn restarting(&mut self, _ctx: &mut Self::Context) {
        LOGGER.warn("DataConsumerActor is restarting...");
        self.set_state(ConsumerActorState::Idle);
        self.set_sender(None);
    }
}

//───brigde───────────────────────────────────────────────────────────────────────────

use super::super::general_messages::{SendActorActionMessage, SendActorActionMessageResult};
use super::super::general_ports::SendActionToActor;
use async_trait::async_trait;
pub struct ConsumerActorBridge<T, U>
where
    T: DataSource + Send + Sync + 'static,
    U: SendDataToProcessor + Send + Sync + 'static,
{
    addr: Addr<DataConsumerActor<T, U>>,
}

impl<T, U> ConsumerActorBridge<T, U>
where
    T: DataSource + Send + Sync + 'static,
    U: SendDataToProcessor + Send + Sync + 'static,
{
    pub fn new(addr: Addr<DataConsumerActor<T, U>>) -> Self {
        Self { addr }
    }
}

#[async_trait]
impl<T, U> SendActionToActor for ConsumerActorBridge<T, U>
where
    T: DataSource + Send + Sync + 'static,
    U: SendDataToProcessor + Send + Sync + 'static,
{
    async fn send_stop_actor(&self) -> SendActorActionMessageResult {
        self.addr
            .send(SendActorActionMessage::stop())
            .await
            .map_err(|e| {
                crate::domain::error::PipelineLifecycleError::InternalCommunication {
                    reason: format!("Failed to send stop message to consumer actor: {}", e),
                }
            })?
    }

    async fn send_restart_actor(&self) -> SendActorActionMessageResult {
        self.addr
            .send(SendActorActionMessage::restart())
            .await
            .map_err(|e| {
                crate::domain::error::PipelineLifecycleError::InternalCommunication {
                    reason: format!("Failed to send restart message to consumer actor: {}", e),
                }
            })?
    }

    async fn get_actor_status(&self) -> SendActorActionMessageResult {
        self.addr
            .send(SendActorActionMessage::status())
            .await
            .map_err(|e| {
                crate::domain::error::PipelineLifecycleError::InternalCommunication {
                    reason: format!("Failed to send status message to consumer actor: {}", e),
                }
            })?
    }
}