use super::messages::{SendDataToStoreMessage,StoreActorResult};
use super::data_store_actor::DataStoreActor;
use crate::domain::outbound::data_external_store::DataExternalStore;
use actix::prelude::*;
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

use crate::adapters::actor_system::pipeline_actor_module::general_messages::{
    ActorActions, ResponseActorActionMessage, SendActorActionMessage, SendActorActionMessageResult,
};

impl<T> Handler<SendActorActionMessage> for DataStoreActor<T>
where
    T: DataExternalStore + Send + Sync + 'static,
{
    type Result = ResponseFuture<SendActorActionMessageResult>;

    fn handle(&mut self, msg: SendActorActionMessage, _ctx: &mut Self::Context) -> Self::Result {
        Box::pin(async move {
            match msg.action() {
                ActorActions::Stop => {
                    LOGGER.info("DataStoreActor: Stop action received.");
                    Ok(ResponseActorActionMessage::stopped())
                }
                ActorActions::Restart => {
                    LOGGER.info("DataStoreActor: Restart action received.");
                    Ok(ResponseActorActionMessage::restarting())
                }
                ActorActions::Status => {
                    LOGGER.info("DataStoreActor: Status action received.");
                    Ok(ResponseActorActionMessage::running())
                }
                _ => {
                    LOGGER.warn("DataStoreActor: Unknown action received.");
                    Ok(ResponseActorActionMessage::failed())
                }
            }
        })
    }
}