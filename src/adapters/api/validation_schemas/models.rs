use serde::{Deserialize, Serialize};
use validator::Validate;
use serde_json::Value;

use crate::domain::error::PipelinePersistenceError;

#[derive(Deserialize, Validate)]
pub struct CreateValidationSchemaRequest {
    #[serde(rename = "jsonName")]
    #[validate(length(min = 1, max = 32))]
    pub name: String,
    #[serde(rename = "jsonSchema")]
    #[validate(length(min = 2, max = 2048))] // por definir el maximo de caracteres permitidos para el json 
    pub json_schema: String,
}

impl CreateValidationSchemaRequest{
    pub fn validate_values(&self)-> Result<(), PipelinePersistenceError> {
        self.validate().map_err(|e| PipelinePersistenceError::InvalidData { reason: e.to_string() })?; 

        serde_json::from_str::<Value>(&self.json_schema)
            .map_err(|e| PipelinePersistenceError::InvalidData { 
                reason: format!("Invalid JSON schema: {}", e) 
            })?;
    
        Ok(())
    
    }
}



#[derive(Deserialize, Validate)]
pub struct UpdateValidationSchemaRequest {
    #[serde(rename = "jsonName")]
    #[validate(length(min = 1, max = 32))]
    pub name: String,
    #[serde(rename = "jsonSchema")]
    #[validate(length(min = 2, max = 2048))]
    pub json_schema: String,
}

#[derive(Serialize)]
pub struct ValidationSchemaResponse {
    pub id: i32,
    pub name: String,
    pub json_schema: String,
    pub created_at: String,
    pub updated_at: String,
}
