use crate::adapters::actor_system::pipeline_actor_module::general_ports::SendDataToStore;
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

impl<T: SendDataToStore + Send + Sync + 'static> Handler<ProcessDataMessage>
    for DataProcessorActor<T>
{
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



use super::super::general_messages::{SendActorActionMessage,SendActorActionMessageResult,ResponseActorActionMessage,ActorActions};

impl<T: SendDataToStore + Send + Sync + 'static> Handler<SendActorActionMessage>
    for DataProcessorActor<T>
{
    type Result = ResponseFuture<SendActorActionMessageResult>;

    fn handle(&mut self, msg: SendActorActionMessage, ctx: &mut Self::Context) -> Self::Result {
        
        Box::pin(async move {
            // Aquí puedes agregar la lógica para manejar el mensaje de acción
            LOGGER.info(&format!("Received action message: {:?}", msg.action()));
            // Por ejemplo, podrías iniciar o detener el procesamiento según la acción recibida
            match msg.action() {
                ActorActions::Start => {
                    LOGGER.info("Starting data processing...");
                    // Lógica para iniciar el procesamiento
                    Ok(ResponseActorActionMessage::running())
                }
                ActorActions::Stop => {
                    LOGGER.info("Stopping data processing...");
                    // Lógica para detener el procesamiento
                    Ok(ResponseActorActionMessage::stopped())
                }
                ActorActions::Restart => {
                    LOGGER.info("Restarting data processing...");
                    // Lógica para reiniciar el procesamiento
                    Ok(ResponseActorActionMessage::restarting())
                }
                ActorActions::Status => {
                    LOGGER.info("Checking data processing status...");
                    Ok(ResponseActorActionMessage::running()) // Aquí puedes devolver el estado actual real
                }
                _ => {
                    LOGGER.warn("Received unknown action.");
                    Ok(ResponseActorActionMessage::failed())
                }
            }
        })        
    }
}