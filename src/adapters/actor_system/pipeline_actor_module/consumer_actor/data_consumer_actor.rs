use actix::prelude::*;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

use crate::domain::outbound::data_source::DataSource;
use crate::domain::entities::data_consumer_types::DataConsumerRawType;
use crate::adapters::actor_system::pipeline_actor_module::consumer_actor::messages::{
    ConsumerActorActionMessage, ConsumerActorState,
};
use crate::logging::AppLogger;

static LOGGER: AppLogger = AppLogger::new("iot_bee::adapters::actor_system::pipeline_actor_module::consumer_actor::DataConsumerActor");

pub struct DataConsumerActor<T: DataSource + Send + Sync + 'static> {
    pub data_source: Arc<T>,
    pub state: ConsumerActorState,
    pub sender: Option<Sender<DataConsumerRawType>>,
}

impl<T: DataSource + Send + Sync + 'static> DataConsumerActor<T> {
    pub fn new(data_source: Arc<T>) -> Self {
        DataConsumerActor {
            data_source,
            state: ConsumerActorState::Idle,
            sender: None,
        }
    }
    pub fn data_source(&self) -> Arc<T> {
        Arc::clone(&self.data_source)
    }
}

impl<T: DataSource + Send + Sync + 'static> Actor for DataConsumerActor<T> {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        LOGGER.info("DataConsumerActor started, initiating data consumption...");
        ctx.address().do_send(ConsumerActorActionMessage::start_consuming());
    }
    fn stopped(&mut self, ctx: &mut Self::Context) {
        LOGGER.info("DataConsumerActor stopped.");
    }
}

impl<T: DataSource + Send + Sync + 'static> Supervised for DataConsumerActor<T> {
    fn restarting(&mut self, ctx: &mut Self::Context) {
        LOGGER.warn("DataConsumerActor is restarting...");
        self.state = ConsumerActorState::Idle;
        self.sender = None;
    }
}