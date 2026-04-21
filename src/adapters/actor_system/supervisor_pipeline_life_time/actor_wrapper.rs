use super::pipeline_supervisor::PipelineSupervisor;
use super::messges::{
    AddPipelineMessage, ListPipelinesMessage, RemovePipelineMessage, RestartPipelineMessage,
    StatusPipelineMessage, StopPipelineMessage,
};
use actix::prelude::*;

// ── SupervisorPipelineBridge ──────────────────────────────────────────────────
//
// Wrapper de Addr<PipelineSupervisor> que expone la API del supervisor
// a través de métodos async sin exponer el tipo Actor al exterior.

pub struct SupervisorPipelineBridge {
    addr: Addr<PipelineSupervisor>,
}

impl SupervisorPipelineBridge {
    pub fn new(addr: Addr<PipelineSupervisor>) -> Self {
        Self { addr }
    }

    pub fn addr(&self) -> &Addr<PipelineSupervisor> {
        &self.addr
    }
}
