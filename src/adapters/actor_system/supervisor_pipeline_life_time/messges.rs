use actix::prelude::*;

use super::super::pipeline_actor_module::general_ports::SendActionToActor;
use super::pipeline_abstraction::{PipelineAbstractionController, TripleResult};
use crate::domain::error::IoTBeeError;

// ── AddPipeline ───────────────────────────────────────────────────────────────
// Registra un nuevo pipeline en el supervisor. Operación síncrona.

pub struct AddPipelineMessage<T, U, V>
where
    T: SendActionToActor,
    U: SendActionToActor,
    V: SendActionToActor,
{
    pub id: u32,
    pub controller: PipelineAbstractionController<T, U, V>,
}

impl<T, U, V> AddPipelineMessage<T, U, V>
where
    T: SendActionToActor,
    U: SendActionToActor,
    V: SendActionToActor,
{
    pub fn new(id: u32, controller: PipelineAbstractionController<T, U, V>) -> Self {
        Self { id, controller }
    }
}

impl<T, U, V> Message for AddPipelineMessage<T, U, V>
where
    T: SendActionToActor,
    U: SendActionToActor,
    V: SendActionToActor,
{
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
