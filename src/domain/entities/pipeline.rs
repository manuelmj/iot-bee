
use crate::domain::value_objects::pipelines_values::DataStroreId; 
//imports relate to pipeline data source 
use crate::domain::value_objects::pipelines_values::{ConnectionType, PipelineDataSourceId, PipilineDataSourceConnection};
use chrono::{DateTime, Utc}; 

use crate::domain::error::IoTBeeError;


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

    pub fn id(&self) -> u32 {
        self.id.id()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn schema(&self) -> &str {
        &self.schema.schema()
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

}


pub struct PipelineNewValidateSchema{
    pub name: String,
    pub schema: PipelineSchemaModel, 
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PipelineNewValidateSchema {

    pub fn existing( name: impl Into<String>, schema: impl Into<String>, created_at: DateTime<Utc>, updated_at: DateTime<Utc>) -> 
    Result<Self, IoTBeeError> {
        let schema = PipelineSchemaModel::new(schema.into())?;
        
        Ok(PipelineNewValidateSchema {
            name: name.into(),
            schema,
            created_at,
            updated_at,
        })
        
    }

    pub fn new(name: impl Into<String>, schema: impl Into<String>) -> Result<Self, IoTBeeError> {
        let now = Utc::now();
        let schema = PipelineSchemaModel::new(schema.into())?; // Aquí se asume que la creación del PipelineSchemaModel no fallará, pero en un caso real deberías manejar el error adecuadamente.
        Ok(PipelineNewValidateSchema {
            name: name.into(),
            schema,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn name (&self) -> &String {
        &self.name
    }
    pub fn schema(&self) -> &str {
        &self.schema.schema()
    }
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at

    }
    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }



}






