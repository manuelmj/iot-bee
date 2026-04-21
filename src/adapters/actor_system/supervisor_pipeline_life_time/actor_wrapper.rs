use actix::prelude::*;

use super::messges::{
    AddReplicaMessage, RemoveReplicaMessage, ReplicaCountMessage, RestartAllReplicasMessage,
    StatusAllReplicasMessage, StopAllReplicasMessage,
};
use super::pipeline_abstraction::{AllReplicasResult, PipelineAbstractionController};
use super::pipeline_supervisor::PipelineSupervisor;
use crate::domain::error::{IoTBeeError, PipelineLifecycleError};

// ── SupervisorPipelineBridge ──────────────────────────────────────────────────
//
// Wrapper cloneable de Addr<PipelineSupervisor>. Expone la API del supervisor
// como métodos async tipados sin exponer el tipo Actor al exterior.
// Addr<A> implementa Clone, por lo que el derive es suficiente.

#[derive(Clone)]
pub struct SupervisorPipelineBridge {
    addr: Addr<PipelineSupervisor>,
}

impl SupervisorPipelineBridge {
    pub fn new(addr: Addr<PipelineSupervisor>) -> Self {
        Self { addr }
    }

    pub async fn add_replica(
        &self,
        controller: PipelineAbstractionController,
    ) -> Result<usize, IoTBeeError> {
        self.addr
            .send(AddReplicaMessage::new(controller))
            .await
            .map_err(mailbox_err)?
    }

    pub async fn remove_replica(&self) -> Result<(), IoTBeeError> {
        self.addr
            .send(RemoveReplicaMessage)
            .await
            .map_err(mailbox_err)?
    }

    pub async fn replica_count(&self) -> Result<usize, IoTBeeError> {
        self.addr
            .send(ReplicaCountMessage)
            .await
            .map_err(mailbox_err)?
    }

    pub async fn stop_all(&self) -> Result<AllReplicasResult, IoTBeeError> {
        self.addr
            .send(StopAllReplicasMessage)
            .await
            .map_err(mailbox_err)?
    }

    pub async fn restart_all(&self) -> Result<AllReplicasResult, IoTBeeError> {
        self.addr
            .send(RestartAllReplicasMessage)
            .await
            .map_err(mailbox_err)?
    }

    pub async fn status_all(&self) -> Result<AllReplicasResult, IoTBeeError> {
        self.addr
            .send(StatusAllReplicasMessage)
            .await
            .map_err(mailbox_err)?
    }
}

fn mailbox_err(e: MailboxError) -> IoTBeeError {
    PipelineLifecycleError::InternalCommunication {
        reason: format!("Fallo de comunicación con PipelineSupervisor: {}", e),
    }
    .into()
}
