
use crate::domain::value_objects::pipelines_values::DataStroreId; 
//imports relate to pipeline data source 
use crate::domain::value_objects::pipelines_values::{ConnectionType, PipelineDataSourceId, PipilineDataSourceConnection};
use chrono::{DateTime, Utc}; 


//imports relate to validation schemas
use crate::domain::value_objects::pipelines_values::{PipelineSchemaModel};
// structs relate to pipeline data source 
pub struct PipeLineDataSourceModel{
    id : DataStroreId,  
    name: String,
    source_type: ConnectionType, 
    source_connection: PipilineDataSourceConnection,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}



// structs relate to validations schemas 
pub struct PipelineValidationSchemaModel{
    id: DataStroreId,
    name: String,
    schema: PipelineSchemaModel, 
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl PipelineValidationSchemaModel {
    pub fn new(id: DataStroreId, name: String, schema: PipelineSchemaModel, created_at: DateTime<Utc>, updated_at: DateTime<Utc>) -> Self {
        Self { id, name, schema, created_at, updated_at }
    }
}
pub struct PipelineNewValidateSchema{
    pub name: String,
    pub schema: PipelineSchemaModel, 
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PipelineNewValidateSchema {

    pub fn existing( name: String, schema: PipelineSchemaModel, created_at: DateTime<Utc>, updated_at: DateTime<Utc>) -> Self {
        PipelineNewValidateSchema {
            name,
            schema,
            created_at,
            updated_at,
        }
    }

    pub fn new(name: String, schema: PipelineSchemaModel) -> Self {
        let now = Utc::now();
        PipelineNewValidateSchema {
            name,
            schema,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn name (&self) -> &String {
        &self.name
    }
    pub fn schema(&self) -> &PipelineSchemaModel {
        &self.schema
    }
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at

    }
    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }



}






