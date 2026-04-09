use std::string;

use actix_web::cookie::time::format_description;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use validator::Validate;

use crate::domain::error::PipelinePersistenceError;

#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateValidationSchemaRequest {
    #[serde(rename = "jsonName")]
    #[validate(length(min = 1, max = 32))]
    pub name: String,
    #[serde(rename = "jsonSchema")]
    #[validate(length(min = 2, max = 2048))]
    // por definir el maximo de caracteres permitidos para el json
    pub json_schema: String,
}

impl CreateValidationSchemaRequest {
    pub fn validate_values(&self) -> Result<(), PipelinePersistenceError> {
        self.validate()
            .map_err(|e| PipelinePersistenceError::InvalidData {
                reason: e.to_string(),
            })?;

        serde_json::from_str::<Value>(&self.json_schema).map_err(|e| {
            PipelinePersistenceError::InvalidData {
                reason: format!("Invalid JSON schema: {}", e),
            }
        })?;

        Ok(())
    }
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct UpdateValidationSchemaRequestName {
    #[serde(rename = "SchemaName")]
    #[validate(length(min = 1, max = 32))]
    pub name: String,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct UpdateValidationSchemaRequestJson {
    #[serde(rename = "jsonSchema")]
    #[validate(length(min = 2, max = 2048))]
    pub json_schema: String,
}

impl UpdateValidationSchemaRequestJson {
    pub fn validate_values(&self) -> Result<(), PipelinePersistenceError> {
        self.validate()
            .map_err(|e| PipelinePersistenceError::InvalidData {
                reason: e.to_string(),
            })?;

        serde_json::from_str::<Value>(&self.json_schema).map_err(|e| {
            PipelinePersistenceError::InvalidData {
                reason: format!("Invalid JSON schema: {}", e),
            }
        })?;

        Ok(())
    }
    pub fn json_schema(&self) -> &str {
        &self.json_schema
    }
}

#[derive(Serialize, ToSchema)]
pub struct ValidationSchemaResponse {
    pub id: u32,
    pub name: String,
    pub json_schema: String,
    pub created_at: String,
    pub updated_at: String,
}

pub type SchemaId = u32;

#[derive(Serialize, ToSchema)]
pub struct ValidationSchemaByIdResponse {
    pub name: String,
    pub json_schema: String,
    pub created_at: String,
    pub updated_at: String,
}
