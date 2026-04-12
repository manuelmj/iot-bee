pub mod data_source;
pub mod pipeline_lifecycle;
pub mod pipeline_persistence;

use pipeline_persistence::{
    PipelineConnectionTypeRepository, 
    PipelineValidationSchemaRepository,
    PipelineDataSourceRepository,
    PipelineGroupRepository
};

#[async_trait::async_trait]
pub trait PipelineGeneralRepository:
    PipelineValidationSchemaRepository + PipelineConnectionTypeRepository + PipelineDataSourceRepository + PipelineGroupRepository
{
}
