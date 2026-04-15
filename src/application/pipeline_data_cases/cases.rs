use crate::domain::error::{IoTBeeError,PipelinePersistenceError};
use crate::domain::outbound::pipeline_persistence::PipelineControllerRepository;
use crate::domain::entities::pipeline_data::{PipelineDataInputModel, PipelineDataOutputModel};
use crate::domain::value_objects::pipelines_values::DataStroreId;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait PipelineDataUseCases {
    async fn create_pipeline(&self, pipeline: &PipelineDataInputModel) -> Result<(), IoTBeeError>;
    async fn get_pipeline(&self) -> Result<Vec<PipelineDataOutputModel>, IoTBeeError>;
    async fn get_pipeline_by_id(&self, pipeline_id: &u32) -> Result<PipelineDataOutputModel, IoTBeeError>;
}

pub struct PipelineDataUseCasesImpl<T: PipelineControllerRepository + Send + Sync> {
    repository: Arc<T>,
}
impl <T: PipelineControllerRepository + Send + Sync> PipelineDataUseCasesImpl<T> {
    pub fn new(repository: Arc<T>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<T> PipelineDataUseCases for PipelineDataUseCasesImpl<T>
where
    T: PipelineControllerRepository + Send + Sync,
{
    async fn create_pipeline(&self, pipeline: &PipelineDataInputModel) -> Result<(), IoTBeeError> {
        self.repository.save_pipeline(pipeline).await
    }
    async fn get_pipeline(&self) -> Result<Vec<PipelineDataOutputModel>, IoTBeeError> {
        self.repository.get_pipeline().await
    }
    async fn get_pipeline_by_id(&self, pipeline_id: &u32) -> Result<PipelineDataOutputModel, IoTBeeError> {
        let pipeline_id = DataStroreId::new(*pipeline_id)?;
        let result = self
            .repository
            .get_pipeline_by_id(&pipeline_id)
            .await?;

        match result {
            Some(pipeline) => Ok(pipeline),
            None => Err(PipelinePersistenceError::IdNotFound { id: pipeline_id.id() }.into()),
        }
    }
}