use crate::domain::entities::pipeline_groups::{PipelineGroupInputModel, PipelineGroupOutputModel};
use crate::domain::error::{IoTBeeError, PipelinePersistenceError};
use crate::domain::outbound::PipelineGeneralRepository;
use crate::domain::value_objects::pipelines_values::DataStroreId;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait PipelineGroupUseCases {
    //with life time anotations
    async fn create_pipeline_group(
        &self,
        group_name: &str,
        group_description: &str,
    ) -> Result<(), IoTBeeError>;

    async fn get_pipeline_groups(&self) -> Result<Vec<PipelineGroupOutputModel>, IoTBeeError>;
    async fn get_pipeline_group_by_id(
        &self,
        group_id: &u32,
    ) -> Result<PipelineGroupOutputModel, IoTBeeError>;
    // async fn delete_pipeline_group(&self, group_id: &u32) -> Result<(), IoTBeeError>;
}

pub struct PipelineGroupUseCasesImpl<T: PipelineGeneralRepository + Send + Sync> {
    repository: Arc<T>,
}
impl<T: PipelineGeneralRepository + Send + Sync> PipelineGroupUseCasesImpl<T> {
    pub fn new(repository: Arc<T>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<T> PipelineGroupUseCases for PipelineGroupUseCasesImpl<T>
where
    T: PipelineGeneralRepository + Send + Sync,
{
    async fn create_pipeline_group(
        &self,
        group_name: &str,
        group_description: &str,
    ) -> Result<(), IoTBeeError> {
        let group_input = PipelineGroupInputModel::new(group_name, group_description)?;
        self.repository.save_pipeline_group(&group_input).await
    }

    async fn get_pipeline_groups(&self) -> Result<Vec<PipelineGroupOutputModel>, IoTBeeError> {
        self.repository.get_pipeline_group().await
    }

    async fn get_pipeline_group_by_id(
        &self,
        group_id: &u32,
    ) -> Result<PipelineGroupOutputModel, IoTBeeError> {
        let group_id = DataStroreId::new(*group_id)?;
        let result = self.repository.get_pipeline_group_by_id(&group_id).await?;

        if let Some(group) = result {
            Ok(group)
        } else {
            Err(PipelinePersistenceError::IdNotFound { id: group_id.id() }.into())
        }
    }
}
