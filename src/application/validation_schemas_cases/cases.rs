use crate::domain::entities::validation_schema::{
    PipelineNewValidateSchema, PipelineValidationSchemaModel,
};
use crate::domain::error::{IoTBeeError, PipelinePersistenceError};
use crate::domain::outbound::pipeline_persistence::PipelineValidationSchemaRepository;
use crate::domain::value_objects::pipelines_values::DataStroreId;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait SchemaValidationUseCases {
    async fn create_validation_schema(&self, name: &str, schema: &str) -> Result<(), IoTBeeError>;

    async fn update_validation_schema(
        &self,
        schema_id: u32,
        new_schema: &str,
    ) -> Result<(), IoTBeeError>;

    async fn update_validation_schema_name(
        &self,
        schema_id: u32,
        new_name: &str,
    ) -> Result<(), IoTBeeError>;

    async fn get_validation_schema(
        &self,
    ) -> Result<Vec<PipelineValidationSchemaModel>, IoTBeeError>;

    async fn get_validation_schema_by_id(
        &self,
        id: u32,
    ) -> Result<PipelineNewValidateSchema, IoTBeeError>;
}

pub struct SchemaValidationUseCasesImpl<T: PipelineValidationSchemaRepository + Send + Sync> {
    repository: Arc<T>,
}

impl<T: PipelineValidationSchemaRepository + Send + Sync> SchemaValidationUseCasesImpl<T> {
    pub fn new(repository: Arc<T>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<T> SchemaValidationUseCases for SchemaValidationUseCasesImpl<T>
where
    T: PipelineValidationSchemaRepository + Send + Sync,
{
    async fn create_validation_schema(&self, name: &str, schema: &str) -> Result<(), IoTBeeError> {
        let domain_schema = PipelineNewValidateSchema::new(name, schema)?;

        self.repository
            .save_pipeline_validation_schema(&domain_schema)
            .await?;

        Ok(())
    }

    async fn get_validation_schema(
        &self,
    ) -> Result<Vec<PipelineValidationSchemaModel>, IoTBeeError> {
        self.repository.list_pipeline_validation_schema().await
    }

    async fn get_validation_schema_by_id(
        &self,
        id: u32,
    ) -> Result<PipelineNewValidateSchema, IoTBeeError> {
        let id_to_search = DataStroreId::new(id)?;

        let result = self
            .repository
            .get_pipeline_validation_schema(&id_to_search)
            .await?;

        if result.is_none() {
            return Err(PipelinePersistenceError::IdNotFound {
                id: id_to_search.id(),
            }
            .into());
        }
        Ok(result.unwrap())
    }

    async fn update_validation_schema_name(
        &self,
        schema_id: u32,
        new_name: &str,
    ) -> Result<(), IoTBeeError> {
        let id_to_search = DataStroreId::new(schema_id)?;
        let result = self
            .repository
            .get_pipeline_validation_schema(&id_to_search)
            .await?;
        if result.is_none() {
            return Err(PipelinePersistenceError::ValidationSchemaNotFound {
                schema_id: schema_id.to_string(),
            }
            .into());
        }
        self.repository
            .update_pipeline_validation_schema_name(&id_to_search, new_name)
            .await?;

        Ok(())
    }

    async fn update_validation_schema(
        &self,
        schema_id: u32,
        new_schema: &str,
    ) -> Result<(), IoTBeeError> {
        let id_to_search = DataStroreId::new(schema_id)?;
        let result = self
            .repository
            .get_pipeline_validation_schema(&id_to_search)
            .await?;
        if result.is_none() {
            return Err(IoTBeeError::from(
                PipelinePersistenceError::ValidationSchemaNotFound {
                    schema_id: schema_id.to_string(),
                },
            ));
        }
        let new_schema = PipelineNewValidateSchema::new("", new_schema)?;
        self.repository
            .update_pipeline_validation_schema(&id_to_search, &new_schema)
            .await?;

        //TODO: Realizar el reinicio de los pipelines que utilizan este schema de validación, para que tomen el nuevo schema actualizado

        Ok(())
    }
}
