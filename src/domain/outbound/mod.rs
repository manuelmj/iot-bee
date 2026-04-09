pub mod data_source;
pub mod pipeline_lifecycle;
pub mod pipeline_persistence;

use pipeline_persistence::{PipelineValidationSchemaRepository, PipelineLifecycleRepository};


#[async_trait::async_trait]
pub trait PipelineGeneralRepository: PipelineValidationSchemaRepository //+ PipelineLifecycleRepository
{}
