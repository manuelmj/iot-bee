use super::messages::{SendDataToStoreMessage,StoreActorResult};
use super::data_store_actor::DataStoreActor;
use crate::domain::outbound::data_external_store::DataExternalStore;
// use crate::domain::entities::data_consumer_types::DataConsumerRawType;
// use crate::domain::error::{IoTBeeError,PipelinePersistenceError};
use actix::prelude::*;
// use crate::adapters::actor_system::pipeline_actor_module::general_ports::SendDataToStore;
use crate::logging::AppLogger;

static LOGGER: AppLogger = AppLogger::new(
    "iot_bee::adapters::actor_system::pipeline_actor_module::store_actor::handlers",
);  

use async_trait::async_trait;


#[async_trait]
impl<T> Handler<SendDataToStoreMessage> for DataStoreActor<T>
where
    T: DataExternalStore + Send + Sync + 'static,
{
    type Result = ResponseFuture<StoreActorResult>;

    fn handle(&mut self, msg: SendDataToStoreMessage, _ctx: &mut Context<Self>) -> Self::Result {
        let data = msg.data().clone();
        let external_store = self.external_store();

        Box::pin(
            async move {
                //TODO: agregar el manejo de errores el, retray y las ultimas validaciones de datos antes de insertar en el store. 
                LOGGER.info("Received SendDataToStoreMessage, saving data to external store...");
                external_store.save(data).await
            } 
        )
    }
}