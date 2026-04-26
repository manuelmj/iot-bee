use crate::adapters::actor_system::pipeline_actor_module::processor_actor::data_processor_actor::DataProcessorActor;
use crate::adapters::actor_system::pipeline_actor_module::processor_actor::messages::{
    ProcessDataMessage, ProcessDataResult,
};

use crate::logging::AppLogger;
use actix::prelude::*;
// use crate::domain::error::IoTBeeError;
use crate::domain::error::PipelineLifecycleError;
static LOGGER: AppLogger = AppLogger::new(
    "iot_bee::adapters::actor_system::pipeline_actor_module::processor_actor::handlers",
);

impl Handler<ProcessDataMessage> for DataProcessorActor {
    type Result = ResponseFuture<ProcessDataResult>;

    fn handle(&mut self, msg: ProcessDataMessage, _ctx: &mut Self::Context) -> Self::Result {
        let data = msg.data();
        // Aquí puedes agregar la lógica para procesar los datos recibidos
        LOGGER.info(&format!("Processing data: {:?}", data));
        let data_store = self.data_store();

        Box::pin(async move {
            //TODO: Aqui es donde se procesarian los datos antes de enviarlos al store, por ahora solo los envio tal cual
            //aca se deben filtrar los datos, crear las transformaciones necesarias, y validaciones.
            // tambien se debe agregar trazabilidad y observabilidad en esta seccion.

            data_store.send(msg.data()).await.map_err(|e| {
                LOGGER.error(&format!("Failed to send data to store: {}", e));
                PipelineLifecycleError::InternalCommunication {
                    reason: format!("Failed to send data to store: {}", e),
                }
                .into()
            })
        })
    }
}

use super::super::general_messages::{
    ActorActions, ResponseActorActionMessage, SendActorActionMessage, SendActorActionMessageResult,
};

impl Handler<SendActorActionMessage> for DataProcessorActor {
    type Result = ResponseFuture<SendActorActionMessageResult>;

    fn handle(&mut self, msg: SendActorActionMessage, ctx: &mut Self::Context) -> Self::Result {
        LOGGER.info(&format!("Received action message: {:?}", msg.action()));
        match msg.action() {
            ActorActions::Stop => {
                LOGGER.info("Stopping data processing...");
                ctx.stop();
                Box::pin(async {
                    LOGGER.info("DataProcessorActor stopped");
                    Ok(ResponseActorActionMessage::stopped())
                })
            }
            ActorActions::Restart => {
                LOGGER.info("Restarting data processing...");
                Box::pin(async {
                    LOGGER.info("DataProcessorActor restarting");
                    Ok(ResponseActorActionMessage::restarting())
                })
            }
            ActorActions::Status => {
                LOGGER.info("Checking data processing status...");
                Box::pin(async {
                    LOGGER.info("DataProcessorActor running");
                    Ok(ResponseActorActionMessage::running())
                })
            }
        }
    }
}
