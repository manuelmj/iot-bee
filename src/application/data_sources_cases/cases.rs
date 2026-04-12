use crate::domain::entities::data_source::{
    PipelineDataSourceInputModel, PipelineDataSourceOutputModel,
    PipelineDataSourceUpdateModel,
};
use crate::domain::value_objects::pipelines_values::{DataStroreId,FieldName};

use crate::domain::error::{IoTBeeError, PipelinePersistenceError};
use crate::domain::outbound::PipelineGeneralRepository;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait DataSourcesUseCases {
    async fn create_data_source(
        &self,
        data_source: &PipelineDataSourceInputModel,
    ) -> Result<(), IoTBeeError>;
    // async fn delete_data_source(&self) -> Result<(), IoTBeeError>;
    async fn update_data_source(&self, data_source_id: &u32, data: &PipelineDataSourceUpdateModel) -> Result<(), IoTBeeError>;
    async fn update_data_source_name(&self,data_source_id: &u32, new_name: &str) -> Result<(), IoTBeeError>;
    async fn get_data_source(
        &self,
        data_source_id: &u32,
    ) -> Result<PipelineDataSourceOutputModel, IoTBeeError>;
    async fn list_data_sources(&self) -> Result<Vec<PipelineDataSourceOutputModel>, IoTBeeError>;
}

pub struct DataSourcesUseCasesImpl<T: PipelineGeneralRepository + Send + Sync> {
    repository: Arc<T>,
}
impl<T: PipelineGeneralRepository + Send + Sync> DataSourcesUseCasesImpl<T> {
    pub fn new(repository: Arc<T>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<T> DataSourcesUseCases for DataSourcesUseCasesImpl<T>
where
    T: PipelineGeneralRepository + Send + Sync,
{
    async fn create_data_source(
        &self,
        data_source: &PipelineDataSourceInputModel,
    ) -> Result<(), IoTBeeError> {
        self.repository.save_pipeline_data_source(data_source).await
    }
    async fn get_data_source(
        &self,
        data_source_id: &u32,
    ) -> Result<PipelineDataSourceOutputModel, IoTBeeError> {
        let data_source_id = DataStroreId::new(*data_source_id)?;
        let result = self
            .repository
            .get_pipeline_data_source(&data_source_id)
            .await?;

        if result.is_none() {
            return Err(PipelinePersistenceError::IdNotFound {
                id: data_source_id.id(),
            }
            .into());
        }

        Ok(result.unwrap())
    }
    async fn list_data_sources(&self) -> Result<Vec<PipelineDataSourceOutputModel>, IoTBeeError> {
        self.repository.list_pipeline_data_source().await
    }

    async fn update_data_source(&self, data_source_id: &u32, data: &PipelineDataSourceUpdateModel) -> Result<(), IoTBeeError> {
        let data_source_id = DataStroreId::new(*data_source_id)?;
        let update_result = self.repository.update_pipeline_data_source(&data_source_id, data).await?;
        //TODO: Crear aca la logica de reiniciar los pipelines que usen esta data source,
        Ok(update_result)
    }
    async fn update_data_source_name(&self, data_source_id: &u32, new_name: &str) -> Result<(), IoTBeeError> {
        let data_source_id = DataStroreId::new(*data_source_id)?;
        let field_name = FieldName::new(new_name)?;
        self.repository.update_pipeline_data_source_name(&data_source_id, &field_name).await
    }

}
