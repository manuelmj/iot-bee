use actix::prelude::*;

use super::messages::{
    CreatePipelineMessage, DeletePipelineMessage, ListPipelinesMessage, RestartPipelineMessage,
    StatusPipelineMessage, StopPipelineMessage, SystemAddReplicaMessage, SystemRemoveReplicaMessage,
};
use super::super::supervisor_pipeline_life_time::pipeline_abstraction::AllReplicasResult;
use super::system_supervisor::SystemActorSupervisor;
use crate::domain::error::{IoTBeeError, PipelineLifecycleError};

fn not_found(pipeline_id: u32) -> IoTBeeError {
    PipelineLifecycleError::NotFound {
        pipeline_id: pipeline_id.to_string(),
    }
    .into()
}

// ── CreatePipeline ── síncrono ────────────────────────────────────────────────

impl Handler<CreatePipelineMessage> for SystemActorSupervisor {
    type Result = Result<(), IoTBeeError>;

    fn handle(&mut self, msg: CreatePipelineMessage, _ctx: &mut Context<Self>) -> Self::Result {
        self.create_pipeline(msg.pipeline_id)
    }
}

// ── DeletePipeline ── síncrono ────────────────────────────────────────────────

impl Handler<DeletePipelineMessage> for SystemActorSupervisor {
    type Result = Result<(), IoTBeeError>;

    fn handle(&mut self, msg: DeletePipelineMessage, _ctx: &mut Context<Self>) -> Self::Result {
        self.remove_pipeline(msg.pipeline_id)
            .map(|_| ())
            .ok_or_else(|| not_found(msg.pipeline_id))
    }
}

// ── ListPipelines ── síncrono ─────────────────────────────────────────────────

impl Handler<ListPipelinesMessage> for SystemActorSupervisor {
    type Result = Vec<u32>;

    fn handle(&mut self, _msg: ListPipelinesMessage, _ctx: &mut Context<Self>) -> Self::Result {
        self.list_pipeline_ids()
    }
}

// ── SystemAddReplica ── asíncrono ─────────────────────────────────────────────
//
// Patrón: clona el bridge en la parte síncrona (libera el borrow de &self),
// luego delega en el bridge en Box::pin(async move {}).

impl Handler<SystemAddReplicaMessage> for SystemActorSupervisor {
    type Result = ResponseFuture<Result<usize, IoTBeeError>>;

    fn handle(&mut self, msg: SystemAddReplicaMessage, _ctx: &mut Context<Self>) -> Self::Result {
        let bridge = match self.get_bridge(msg.pipeline_id) {
            Some(b) => b,
            None => return Box::pin(async move { Err(not_found(msg.pipeline_id)) }),
        };
        Box::pin(async move { bridge.add_replica(msg.controller).await })
    }
}

// ── SystemRemoveReplica ── asíncrono ──────────────────────────────────────────

impl Handler<SystemRemoveReplicaMessage> for SystemActorSupervisor {
    type Result = ResponseFuture<Result<(), IoTBeeError>>;

    fn handle(
        &mut self,
        msg: SystemRemoveReplicaMessage,
        _ctx: &mut Context<Self>,
    ) -> Self::Result {
        let bridge = match self.get_bridge(msg.pipeline_id) {
            Some(b) => b,
            None => return Box::pin(async move { Err(not_found(msg.pipeline_id)) }),
        };
        Box::pin(async move { bridge.remove_replica().await })
    }
}

// ── StopPipeline ── asíncrono ─────────────────────────────────────────────────

impl Handler<StopPipelineMessage> for SystemActorSupervisor {
    type Result = ResponseFuture<Result<AllReplicasResult, IoTBeeError>>;

    fn handle(&mut self, msg: StopPipelineMessage, _ctx: &mut Context<Self>) -> Self::Result {
        let bridge = match self.get_bridge(msg.pipeline_id) {
            Some(b) => b,
            None => return Box::pin(async move { Err(not_found(msg.pipeline_id)) }),
        };
        Box::pin(async move { bridge.stop_all().await })
    }
}

// ── RestartPipeline ── asíncrono ──────────────────────────────────────────────

impl Handler<RestartPipelineMessage> for SystemActorSupervisor {
    type Result = ResponseFuture<Result<AllReplicasResult, IoTBeeError>>;

    fn handle(&mut self, msg: RestartPipelineMessage, _ctx: &mut Context<Self>) -> Self::Result {
        let bridge = match self.get_bridge(msg.pipeline_id) {
            Some(b) => b,
            None => return Box::pin(async move { Err(not_found(msg.pipeline_id)) }),
        };
        Box::pin(async move { bridge.restart_all().await })
    }
}

// ── StatusPipeline ── asíncrono ───────────────────────────────────────────────

impl Handler<StatusPipelineMessage> for SystemActorSupervisor {
    type Result = ResponseFuture<Result<AllReplicasResult, IoTBeeError>>;

    fn handle(&mut self, msg: StatusPipelineMessage, _ctx: &mut Context<Self>) -> Self::Result {
        let bridge = match self.get_bridge(msg.pipeline_id) {
            Some(b) => b,
            None => return Box::pin(async move { Err(not_found(msg.pipeline_id)) }),
        };
        Box::pin(async move { bridge.status_all().await })
    }
}
