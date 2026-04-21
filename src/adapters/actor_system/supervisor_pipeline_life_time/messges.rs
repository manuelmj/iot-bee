use actix::prelude::*;

use super::pipeline_abstraction::{PipelineAbstractionController, TripleResult};
use crate::domain::error::IoTBeeError;

// ── AddPipeline ───────────────────────────────────────────────────────────────
// Registra un nuevo pipeline en el supervisor. Operación síncrona.

pub struct AddPipelineMessage {
    pub id: u32,
    pub controller: PipelineAbstractionController,
}

impl AddPipelineMessage {
    pub fn new(id: u32, controller: PipelineAbstractionController) -> Self {
        Self { id, controller }
    }
}

impl Message for AddPipelineMessage {
    type Result = Result<(), IoTBeeError>;
}

// ── RemovePipeline ────────────────────────────────────────────────────────────
// Elimina el pipeline del registro. Operación síncrona.

pub struct RemovePipelineMessage {
    pub id: u32,
}

impl RemovePipelineMessage {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}

impl Message for RemovePipelineMessage {
    type Result = Result<(), IoTBeeError>;
}

// ── StopPipeline ──────────────────────────────────────────────────────────────
// Envía stop a los tres actores. Operación asíncrona (ResponseFuture).

pub struct StopPipelineMessage {
    pub id: u32,
}

impl StopPipelineMessage {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}

impl Message for StopPipelineMessage {
    type Result = Result<TripleResult, IoTBeeError>;
}

// ── RestartPipeline ───────────────────────────────────────────────────────────

pub struct RestartPipelineMessage {
    pub id: u32,
}

impl RestartPipelineMessage {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}

impl Message for RestartPipelineMessage {
    type Result = Result<TripleResult, IoTBeeError>;
}

// ── StatusPipeline ────────────────────────────────────────────────────────────

pub struct StatusPipelineMessage {
    pub id: u32,
}

impl StatusPipelineMessage {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}

impl Message for StatusPipelineMessage {
    type Result = Result<TripleResult, IoTBeeError>;
}

// ── ListPipelines ─────────────────────────────────────────────────────────────
// Devuelve los ids de todos los pipelines registrados. Operación síncrona.

pub struct ListPipelinesMessage;

impl Message for ListPipelinesMessage {
    type Result = Result<Vec<u32>, IoTBeeError>;
}
