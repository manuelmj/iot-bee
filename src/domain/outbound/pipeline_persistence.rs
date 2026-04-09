use async_trait::async_trait;


// 
use crate::domain::error::IoTBeeError; 

// entities for validation schema 
use crate::domain::entities::pipeline::{PipelineValidationSchemaModel,PipelineNewValidateSchema};
use crate::domain::value_objects::pipelines_values::DataStroreId; 

 
#[async_trait]
pub trait PipelineLifecycleRepository {
    // these methods are for the pipeline lifecycle
	async fn save_pipeline_lifecycle(&self) -> Result<(), IoTBeeError>;
	async fn delete_pipeline_lifecycle(&self) -> Result<(), IoTBeeError>;
	async fn update_pipeline_lifecycle(&self) -> Result<(), IoTBeeError>;
	async fn get_pipeline_lifecycle(&self) -> Result<Option<String>, IoTBeeError>;
	async fn list_pipeline_lifecycle(&self) -> Result<Vec<String>, IoTBeeError>;
}

    // // these methods are for the data source 
    // async fn save_pipeline_data_source(); 
    // async fn delete_pipeline_data_source();
    // async fn update_pipeline_data_source();
    // async fn get_pipeline_data_source();
    // async fn list_pipeline_data_source();

    // // these methods are for the data store 
    // async fn save_pipeline_data_store(); 
    // async fn delete_pipeline_data_store();
    // async fn update_pipeline_data_store();
    // async fn get_pipeline_data_store();
    // async fn list_pipeline_data_store();

    // // these methods are for the configuration
    // async fn save_pipeline_configuration(); 
    // async fn delete_pipeline_configuration();
    // async fn update_pipeline_configuration();
    // async fn get_pipeline_configuration();
    // async fn list_pipeline_configuration();

#[async_trait]
pub trait PipelineValidationSchemaRepository {
    // these methods are for the validation schema
    async fn save_pipeline_validation_schema(&self, schema: &PipelineNewValidateSchema) -> Result<(), IoTBeeError>; 
    async fn delete_pipeline_validation_schema(&self, schema_id: &DataStroreId) -> Result<(), IoTBeeError>;
    async fn update_pipeline_validation_schema(&self, schema_id: &DataStroreId, schema: &PipelineNewValidateSchema) -> Result<(), IoTBeeError>;
    async fn get_pipeline_validation_schema(&self, schema_id: &DataStroreId) -> Result<Option<PipelineNewValidateSchema>, IoTBeeError>;
    async fn list_pipeline_validation_schema(&self) -> Result<Vec<PipelineValidationSchemaModel>, IoTBeeError>;
    // async fn pipeline_validation_schema_exists_name(&self, schema_name: &str) -> Result<bool, IoTBeeError>;
}