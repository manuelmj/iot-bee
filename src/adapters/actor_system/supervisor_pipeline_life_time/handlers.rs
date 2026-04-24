use actix::prelude::*;

use super::messges::{
    AddReplicaMessage, RemoveReplicaMessage, ReplicaCountMessage, RestartAllReplicasMessage,
    StatusAllReplicasMessage, StopAllReplicasMessage,StartPipelineMessage
};
use super::pipeline_abstraction::AllReplicasResult;
use super::pipeline_supervisor::PipelineSupervisor;
use crate::domain::error::IoTBeeError;



use crate::adapters::actor_system::pipeline_actor_module::store_actor::data_store_actor::{StoreActorBridge};




// ── StartPipeline ─────────────────────────────────────────────

impl Handler<StartPipelineMessage> for PipelineSupervisor {
    type Result = Result<(), IoTBeeError>;

    fn handle(&mut self, _msg: StartPipelineMessage, _ctx: &mut Context<Self>) -> Self::Result {
        //para iniciar los pipelines entonces necesitamos
        //1. data source Actor con su connector de consumo
        //2. data processor Actor con su lógica de procesamiento
        //3. data store Actor con su connector de almacenamiento
        //4. cantidad de replicas iniciales que debe tener el pipeline 
        let replica_count = self.pipeline_configuration().pipeline_replica_count;

        for _ in 0..replica_count {
            
        }

        Ok(())
    }
}

// ── AddReplica ── síncrono ────────────────────────────────────────────────────

impl Handler<AddReplicaMessage> for PipelineSupervisor {
    type Result = Result<usize, IoTBeeError>;

    fn handle(&mut self, msg: AddReplicaMessage, _ctx: &mut Context<Self>) -> Self::Result {
        self.replica_registry().add_replica(msg.controller)
    }
}

// ── RemoveReplica ── síncrono ─────────────────────────────────────────────────

impl Handler<RemoveReplicaMessage> for PipelineSupervisor {
    type Result = Result<(), IoTBeeError>;

    fn handle(&mut self, _msg: RemoveReplicaMessage, _ctx: &mut Context<Self>) -> Self::Result {
        self.replica_registry().remove_last_replica()
    }
}

// ── ReplicaCount ── síncrono ──────────────────────────────────────────────────

impl Handler<ReplicaCountMessage> for PipelineSupervisor {
    type Result = Result<usize, IoTBeeError>;

    fn handle(&mut self, _msg: ReplicaCountMessage, _ctx: &mut Context<Self>) -> Self::Result {
        self.replica_registry().replica_count()
    }
}

// ── StopAllReplicas ── asíncrono ──────────────────────────────────────────────
//
// Patrón: all_arcs() clona los Arc y libera el RwLock en la parte síncrona
// del handler, ANTES de entrar en Box::pin(async move {}).
// El mailbox sigue procesando mensajes mientras el future recorre las réplicas.

impl Handler<StopAllReplicasMessage> for PipelineSupervisor {
    type Result = ResponseFuture<Result<AllReplicasResult, IoTBeeError>>;

    fn handle(&mut self, _msg: StopAllReplicasMessage, _ctx: &mut Context<Self>) -> Self::Result {
        let replicas = match self.replica_registry().all_arcs() {
            Ok(r) => r,
            Err(e) => return Box::pin(async move { Err(e) }),
        };
        Box::pin(async move {
            let mut results = Vec::with_capacity(replicas.len());
            for replica in replicas {
                results.push(replica.stop().await);
            }
            Ok(results)
        })
    }
}

// ── RestartAllReplicas ── asíncrono ───────────────────────────────────────────

impl Handler<RestartAllReplicasMessage> for PipelineSupervisor {
    type Result = ResponseFuture<Result<AllReplicasResult, IoTBeeError>>;

    fn handle(
        &mut self,
        _msg: RestartAllReplicasMessage,
        _ctx: &mut Context<Self>,
    ) -> Self::Result {
        let replicas = match self.replica_registry().all_arcs() {
            Ok(r) => r,
            Err(e) => return Box::pin(async move { Err(e) }),
        };
        Box::pin(async move {
            let mut results = Vec::with_capacity(replicas.len());
            for replica in replicas {
                results.push(replica.restart().await);
            }
            Ok(results)
        })
    }
}

// ── StatusAllReplicas ── asíncrono ────────────────────────────────────────────

impl Handler<StatusAllReplicasMessage> for PipelineSupervisor {
    type Result = ResponseFuture<Result<AllReplicasResult, IoTBeeError>>;

    fn handle(
        &mut self,
        _msg: StatusAllReplicasMessage,
        _ctx: &mut Context<Self>,
    ) -> Self::Result {
        let replicas = match self.replica_registry().all_arcs() {
            Ok(r) => r,
            Err(e) => return Box::pin(async move { Err(e) }),
        };
        Box::pin(async move {
            let mut results = Vec::with_capacity(replicas.len());
            for replica in replicas {
                results.push(replica.status().await);
            }
            Ok(results)
        })
    }
}
