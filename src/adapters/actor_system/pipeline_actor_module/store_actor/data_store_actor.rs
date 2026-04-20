use actix::prelude::*;
use crate::domain::outbound::data_external_store::DataExternalStore;
use crate::domain::entities::data_consumer_types::DataConsumerRawType;
use crate::domain::error::{IoTBeeError,PipelineLifecycleError};
use crate::logging::AppLogger;  
use std::sync::Arc;
use async_trait::async_trait;
use super::messages::SendDataToStoreMessage;

static LOGGER: AppLogger = AppLogger::new(
    "iot_bee::adapters::actor_system::pipeline_actor_module::store_actor::DataStoreActor",
);


pub struct DataStoreActor<T: DataExternalStore + Send + Sync + 'static> {
    external_store: Arc<T>,
}

impl<T: DataExternalStore + Send + Sync + 'static> DataStoreActor<T> {
    pub fn new(external_store: Arc<T>) -> Self {
        Self { external_store }
    }
    pub fn external_store(&self) -> Arc<T> {
        Arc::clone(&self.external_store)
    }
}


impl<T> Actor for DataStoreActor<T>
where
    T: DataExternalStore + Send + Sync + 'static,
{
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        LOGGER.info("DataStoreActor started.");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        LOGGER.info("DataStoreActor stopped.");
    }
}

impl<T> Supervised for DataStoreActor<T>
where
    T: DataExternalStore + Send + Sync + 'static,
{
    fn restarting(&mut self, _ctx: &mut Self::Context) {
        LOGGER.warn("DataStoreActor is restarting.");
    }
}



//────Bridge────────────────────────────────────────────────────────────────────────────────────────────────
//
use super::super::general_ports::SendDataToStore;

pub struct StoreActorBridge<T: DataExternalStore + Send + Sync + 'static> {
    addr: Addr<DataStoreActor<T>>,
}
impl<T> StoreActorBridge<T>
where
    T: DataExternalStore + Send + Sync + 'static,
{
    pub fn new(addr: Addr<DataStoreActor<T>>) -> Self {
        Self { addr }
    }
}

#[async_trait]
impl<T> SendDataToStore for StoreActorBridge<T>
where
    T: DataExternalStore + Send + Sync + 'static,
{
    async fn send(&self, data: &DataConsumerRawType) -> Result<(), IoTBeeError> {
        self.addr
            .send(SendDataToStoreMessage::new(&data))
            .await
            .map_err(|e| {
                PipelineLifecycleError::InternalCommunication {
                    reason: format!("Failed to send message to store actor: {}", e),
                }
            })?
    }
}
