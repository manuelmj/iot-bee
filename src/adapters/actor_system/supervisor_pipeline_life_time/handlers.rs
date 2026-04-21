use actix::prelude::*;

use super::messges::{
    AddPipelineMessage, ListPipelinesMessage, RemovePipelineMessage, RestartPipelineMessage,
    StatusPipelineMessage, StopPipelineMessage,
};
use super::pipeline_abstraction::TripleResult;
use super::pipeline_supervisor::PipelineSupervisor;
use crate::domain::error::IoTBeeError;

// ── AddPipeline ── síncrono ───────────────────────────────────────────────────

impl Handler<AddPipelineMessage> for PipelineSupervisor {
    type Result = Result<(), IoTBeeError>;

    fn handle(
        &mut self,
        msg: AddPipelineMessage,
        _ctx: &mut Context<Self>,
    ) -> Self::Result {
        self.child_registry.add(msg.id, msg.controller)
    }
}

// ── RemovePipeline ── síncrono ────────────────────────────────────────────────

impl Handler<RemovePipelineMessage> for PipelineSupervisor {
    type Result = Result<(), IoTBeeError>;

    fn handle(&mut self, msg: RemovePipelineMessage, _ctx: &mut Context<Self>) -> Self::Result {
        self.child_registry.remove(msg.id).map(|_| ())
    }
}

// ── ListPipelines ── síncrono ─────────────────────────────────────────────────

impl Handler<ListPipelinesMessage> for PipelineSupervisor {
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

impl Handler<StopPipelineMessage> for PipelineSupervisor {
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

impl Handler<RestartPipelineMessage> for PipelineSupervisor {
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

impl Handler<StatusPipelineMessage> for PipelineSupervisor {
    type Result = ResponseFuture<Result<TripleResult, IoTBeeError>>;

    fn handle(&mut self, msg: StatusPipelineMessage, _ctx: &mut Context<Self>) -> Self::Result {
        let controller = match self.child_registry.get_controller(msg.id) {
            Ok(c) => c,
            Err(e) => return Box::pin(async move { Err(e) }),
        };
        Box::pin(async move { Ok(controller.status().await) })
    }
}
