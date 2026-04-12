// Domain errors imports
use crate::domain::error::IoTBeeError;
// Domain entities imports
use crate::domain::entities::connection_type::ConnectionTypeModel;
use crate::domain::entities::data_source::{
    PipelineDataSourceInputModel, PipelineDataSourceOutputModel,PipelineDataSourceUpdateModel
};
use crate::domain::entities::validation_schema::{
    PipelineNewValidateSchema, PipelineValidationSchemaModel,
};
use crate::domain::entities::pipeline_groups::{PipelineGroupInputModel, PipelineGroupOutputModel};


// Domain value objects imports
use crate::domain::value_objects::pipelines_values::{DataStroreId,FieldName};
//general imports
use async_trait::async_trait;


// // these methods are for the data source
#[async_trait]
pub trait PipelineDataSourceRepository {
    async fn save_pipeline_data_source(
        &self,
        data_source: &PipelineDataSourceInputModel,
    ) -> Result<(), IoTBeeError>;
    async fn get_pipeline_data_source(
        &self,
        data_source_id: &DataStroreId,
    ) -> Result<Option<PipelineDataSourceOutputModel>, IoTBeeError>;
    async fn list_pipeline_data_source(
        &self,
    ) -> Result<Vec<PipelineDataSourceOutputModel>, IoTBeeError>;

    async fn update_pipeline_data_source(&self,data_source_id: &DataStroreId, data_source: &PipelineDataSourceUpdateModel) -> Result<(), IoTBeeError>;
    async fn update_pipeline_data_source_name(&self,data_source_id: &DataStroreId, name : &FieldName) -> Result<(), IoTBeeError>;
    /*  async fn delete_pipeline_data_source(&self, data_source_id: &DataStroreId) -> Result<(), IoTBeeError>;
    */
}

// // these methods are for the data store
// #[async_trait]
// pub trait PipelineDataStoreRepository {
    // async fn save_pipeline_data_store(&self, data_store: &String) -> Result<(), IoTBeeError>;
    // async fn delete_pipeline_data_store();
    // async fn update_pipeline_data_store();
    // async fn get_pipeline_data_store();
    // async fn list_pipeline_data_store();
// }

#[async_trait]
pub trait PipelineGroupRepository {
    async fn get_pipeline_group(&self) -> Result<Vec<PipelineGroupOutputModel>, IoTBeeError>;
    async fn get_pipeline_group_by_id(&self, group_id: &DataStroreId) -> Result<Option<PipelineGroupOutputModel>, IoTBeeError>;
    async fn save_pipeline_group(&self, group: &PipelineGroupInputModel) -> Result<(), IoTBeeError>;
    //TODO: add update and delete methods for the pipeline group
    // async fn delete_pipeline_group(&self, group_id: &DataStroreId) -> Result<(), IoTBeeError>;
}



#[async_trait]
pub trait PipelineValidationSchemaRepository {
    // these methods are for the validation schema
    async fn save_pipeline_validation_schema(
        &self,
        schema: &PipelineNewValidateSchema,
    ) -> Result<(), IoTBeeError>;
    async fn delete_pipeline_validation_schema(
        &self,
        schema_id: &DataStroreId,
    ) -> Result<(), IoTBeeError>;
    async fn update_pipeline_validation_schema(
        &self,
        schema_id: &DataStroreId,
        schema: &PipelineNewValidateSchema,
    ) -> Result<(), IoTBeeError>;
    async fn update_pipeline_validation_schema_name(
        &self,
        schema_id: &DataStroreId,
        new_name: &str,
    ) -> Result<(), IoTBeeError>;
    async fn get_pipeline_validation_schema(
        &self,
        schema_id: &DataStroreId,
    ) -> Result<Option<PipelineNewValidateSchema>, IoTBeeError>;
    async fn list_pipeline_validation_schema(
        &self,
    ) -> Result<Vec<PipelineValidationSchemaModel>, IoTBeeError>;
}

#[async_trait]
pub trait PipelineConnectionTypeRepository {
    async fn get_pipeline_connection_type(&self) -> Result<Vec<ConnectionTypeModel>, IoTBeeError>;
}
