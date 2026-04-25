use actix::prelude::*;

use super::pipeline_abstraction::PipelineAbstractionController;
use crate::domain::error::IoTBeeError;

// StartPipeline
// Inicia todos el pipeline
pub struct StartPipelineMessage;
impl Message for StartPipelineMessage {
    type Result = Result<(), IoTBeeError>;
}

// ── AddReplica ────────────────────────────────────────────────────────────────
// Añade una réplica al supervisor. Devuelve el número total de réplicas.

pub struct AddReplicaMessage {
    pub controller: PipelineAbstractionController,
}

impl AddReplicaMessage {
    pub fn new(controller: PipelineAbstractionController) -> Self {
        Self { controller }
    }
}

impl Message for AddReplicaMessage {
    type Result = Result<usize, IoTBeeError>;
}

// ── RemoveReplica ─────────────────────────────────────────────────────────────
// Elimina la última réplica (escala hacia abajo). Error si no hay réplicas.

pub struct RemoveReplicaMessage;

impl Message for RemoveReplicaMessage {
    type Result = Result<(), IoTBeeError>;
}

// ── ReplicaCount ──────────────────────────────────────────────────────────────
// Devuelve el número de réplicas activas.

pub struct ReplicaCountMessage;

impl Message for ReplicaCountMessage {
    type Result = Result<usize, IoTBeeError>;
}

// ── StopAllReplicas ───────────────────────────────────────────────────────────
// Envía stop a todos los actores de todas las réplicas activas.

pub struct StopAllReplicasMessage;

impl Message for StopAllReplicasMessage {
    type Result = Result<(), IoTBeeError>;
}

// ── RestartAllReplicas ────────────────────────────────────────────────────────
// Envía restart a todos los actores de todas las réplicas activas.

pub struct RestartAllReplicasMessage;

impl Message for RestartAllReplicasMessage {
    type Result = Result<(), IoTBeeError>;
}

// ── StatusAllReplicas ─────────────────────────────────────────────────────────
// Consulta el estado de todos los actores de todas las réplicas activas.

pub struct StatusAllReplicasMessage;

impl Message for StatusAllReplicasMessage {
    type Result = Result<(), IoTBeeError>;
}
