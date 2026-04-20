use actix::prelude::*;

use super::super::pipeline_actor_module::general_ports::SendActionToActor;
use super::messges::{
    AddPipelineMessage, ListPipelinesMessage, RemovePipelineMessage, RestartPipelineMessage,
    StatusPipelineMessage, StopPipelineMessage,
};
use super::pipeline_abstraction::TripleResult;
use super::pipeline_supervisor::PipelineSupervisor;
use crate::domain::error::IoTBeeError;

// ── AddPipeline ── síncrono ───────────────────────────────────────────────────

impl<T, U, V> Handler<AddPipelineMessage<T, U, V>> for PipelineSupervisor<T, U, V>
where
    T: SendActionToActor + 'static,
    U: SendActionToActor + 'static,
    V: SendActionToActor + 'static,
{
    type Result = Result<(), IoTBeeError>;

    fn handle(
        &mut self,
        msg: AddPipelineMessage<T, U, V>,
        _ctx: &mut Context<Self>,
    ) -> Self::Result {
        self.child_registry.add(msg.id, msg.controller)
    }
}

// ── RemovePipeline ── síncrono ────────────────────────────────────────────────

impl<T, U, V> Handler<RemovePipelineMessage> for PipelineSupervisor<T, U, V>
where
    T: SendActionToActor + 'static,
    U: SendActionToActor + 'static,
    V: SendActionToActor + 'static,
{
    type Result = Result<(), IoTBeeError>;

    fn handle(&mut self, msg: RemovePipelineMessage, _ctx: &mut Context<Self>) -> Self::Result {
        self.child_registry.remove(msg.id).map(|_| ())
    }
}

// ── ListPipelines ── síncrono ─────────────────────────────────────────────────

impl<T, U, V> Handler<ListPipelinesMessage> for PipelineSupervisor<T, U, V>
where
    T: SendActionToActor + 'static,
    U: SendActionToActor + 'static,
    V: SendActionToActor + 'static,
{
    type Result = Result<Vec<u32>, IoTBeeError>;

    fn handle(&mut self, _msg: ListPipelinesMessage, _ctx: &mut Context<Self>) -> Self::Result {
        self.child_registry.list_ids()
    }
}

// ── StopPipeline ── asíncrono (ResponseFuture) ────────────────────────────────
//
// Patrón: get_controller() clona el Arc y libera el RwLock en la parte
// síncrona del handler, ANTES de entrar en Box::pin(async move {}).
// El mailbox sigue procesando mensajes mientras el future está en vuelo.

impl<T, U, V> Handler<StopPipelineMessage> for PipelineSupervisor<T, U, V>
where
    T: SendActionToActor + 'static,
    U: SendActionToActor + 'static,
    V: SendActionToActor + 'static,
{
    type Result = ResponseFuture<Result<TripleResult, IoTBeeError>>;

    fn handle(&mut self, msg: StopPipelineMessage, _ctx: &mut Context<Self>) -> Self::Result {
        let controller = match self.child_registry.get_controller(msg.id) {
            Ok(c) => c,
            Err(e) => return Box::pin(async move { Err(e) }),
        };
        Box::pin(async move { Ok(controller.stop().await) })
    }
}

// ── RestartPipeline ── asíncrono ──────────────────────────────────────────────

impl<T, U, V> Handler<RestartPipelineMessage> for PipelineSupervisor<T, U, V>
where
    T: SendActionToActor + 'static,
    U: SendActionToActor + 'static,
    V: SendActionToActor + 'static,
{
    type Result = ResponseFuture<Result<TripleResult, IoTBeeError>>;

    fn handle(&mut self, msg: RestartPipelineMessage, _ctx: &mut Context<Self>) -> Self::Result {
        let controller = match self.child_registry.get_controller(msg.id) {
            Ok(c) => c,
            Err(e) => return Box::pin(async move { Err(e) }),
        };
        Box::pin(async move { Ok(controller.restart().await) })
    }
}

// ── StatusPipeline ── asíncrono ───────────────────────────────────────────────

impl<T, U, V> Handler<StatusPipelineMessage> for PipelineSupervisor<T, U, V>
where
    T: SendActionToActor + 'static,
    U: SendActionToActor + 'static,
    V: SendActionToActor + 'static,
{
    type Result = ResponseFuture<Result<TripleResult, IoTBeeError>>;

    fn handle(&mut self, msg: StatusPipelineMessage, _ctx: &mut Context<Self>) -> Self::Result {
        let controller = match self.child_registry.get_controller(msg.id) {
            Ok(c) => c,
            Err(e) => return Box::pin(async move { Err(e) }),
        };
        Box::pin(async move { Ok(controller.status().await) })
    }
}
