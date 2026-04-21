use actix::prelude::*;

use super::super::supervisor_pipeline_life_time::pipeline_abstraction::{
    AllReplicasResult, PipelineAbstractionController,
};
use crate::domain::error::IoTBeeError;

// ── CreatePipeline ────────────────────────────────────────────────────────────
// Crea un nuevo PipelineSupervisor para el pipeline dado.
// Error si ya existe un supervisor para ese pipeline_id.

pub struct CreatePipelineMessage {
    pub pipeline_id: u32,
}

impl CreatePipelineMessage {
    pub fn new(pipeline_id: u32) -> Self {
        Self { pipeline_id }
    }
}

impl Message for CreatePipelineMessage {
    type Result = Result<(), IoTBeeError>;
}

// ── DeletePipeline ────────────────────────────────────────────────────────────
// Elimina el PipelineSupervisor completo para el pipeline dado.
// Error si no existe.

pub struct DeletePipelineMessage {
    pub pipeline_id: u32,
}

impl DeletePipelineMessage {
    pub fn new(pipeline_id: u32) -> Self {
        Self { pipeline_id }
    }
}

impl Message for DeletePipelineMessage {
    type Result = Result<(), IoTBeeError>;
}

// ── ListPipelines ─────────────────────────────────────────────────────────────
// Devuelve los pipeline_ids de todos los supervisores registrados.

pub struct ListPipelinesMessage;

impl Message for ListPipelinesMessage {
    type Result = Vec<u32>;
}

// ── SystemAddReplica ──────────────────────────────────────────────────────────
// Añade una réplica al PipelineSupervisor de un pipeline existente.
// Devuelve el número total de réplicas tras la inserción.

pub struct SystemAddReplicaMessage {
    pub pipeline_id: u32,
    pub controller: PipelineAbstractionController,
}

impl SystemAddReplicaMessage {
    pub fn new(pipeline_id: u32, controller: PipelineAbstractionController) -> Self {
        Self { pipeline_id, controller }
    }
}

impl Message for SystemAddReplicaMessage {
    type Result = Result<usize, IoTBeeError>;
}

// ── SystemRemoveReplica ───────────────────────────────────────────────────────
// Elimina la última réplica del PipelineSupervisor de un pipeline existente.

pub struct SystemRemoveReplicaMessage {
    pub pipeline_id: u32,
}

impl SystemRemoveReplicaMessage {
    pub fn new(pipeline_id: u32) -> Self {
        Self { pipeline_id }
    }
}

impl Message for SystemRemoveReplicaMessage {
    type Result = Result<(), IoTBeeError>;
}

// ── StopPipeline ──────────────────────────────────────────────────────────────
// Envía stop a todas las réplicas del pipeline dado.

pub struct StopPipelineMessage {
    pub pipeline_id: u32,
}

impl StopPipelineMessage {
    pub fn new(pipeline_id: u32) -> Self {
        Self { pipeline_id }
    }
}

impl Message for StopPipelineMessage {
    type Result = Result<AllReplicasResult, IoTBeeError>;
}

// ── RestartPipeline ───────────────────────────────────────────────────────────
// Envía restart a todas las réplicas del pipeline dado.

pub struct RestartPipelineMessage {
    pub pipeline_id: u32,
}

impl RestartPipelineMessage {
    pub fn new(pipeline_id: u32) -> Self {
        Self { pipeline_id }
    }
}

impl Message for RestartPipelineMessage {
    type Result = Result<AllReplicasResult, IoTBeeError>;
}

// ── StatusPipeline ────────────────────────────────────────────────────────────
// Consulta el estado de todas las réplicas del pipeline dado.

pub struct StatusPipelineMessage {
    pub pipeline_id: u32,
}

impl StatusPipelineMessage {
    pub fn new(pipeline_id: u32) -> Self {
        Self { pipeline_id }
    }
}

impl Message for StatusPipelineMessage {
    type Result = Result<AllReplicasResult, IoTBeeError>;
}



// StartAllPipelinesInLocalStorageMessage 

pub struct StartAllPipelinesInLocalStorageMessage;
impl Message for StartAllPipelinesInLocalStorageMessage {
    type Result = Result<(), IoTBeeError>;
}