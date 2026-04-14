pub mod data_source;
pub mod pipeline_lifecycle;
pub mod pipeline_persistence;

use pipeline_persistence::{
    PipelineConnectionTypeRepository, PipelineDataSourceRepository, PipelineGroupRepository,
    PipelineValidationSchemaRepository,PipelineDataStoreRepository,PipelineControllerRepository
};

#[async_trait::async_trait]
pub trait PipelineGeneralRepository:
    PipelineValidationSchemaRepository
    + PipelineConnectionTypeRepository
    + PipelineDataSourceRepository
    + PipelineGroupRepository
    + PipelineDataStoreRepository
    + PipelineControllerRepository
{
}
