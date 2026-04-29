use serde::{Deserialize, Serialize};

use domain::entities::validation_schema::{
    PipelineNewValidateSchema, PipelineValidationSchemaModel,
};

use domain::error::PipelinePersistenceError;
use domain::pipeline_schema::schemas::PipelineSchemaMap;
use chrono::{DateTime, Utc};
use serde_json::Value;
use utoipa::ToSchema;
use validator::Validate;


pub type SchemaId = u32;

#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateValidationSchemaRequest {
    #[serde(rename = "name")]
    #[validate(length(min = 1, max = 32))]
    pub name: String,

    #[serde(rename = "schema")]
    #[validate(custom = "validate_json_schema")]
    pub json_schema: String,
}

fn validate_json_schema(json_str: &str) -> Result<(), validator::ValidationError> {
    serde_json::from_str::<PipelineSchemaMap>(json_str)
        .map_err(|e| {
            let mut err = validator::ValidationError::new("invalid_json_schema");
            err.message = Some(std::borrow::Cow::Owned(format!("Invalid JSON schema: {}", e)));
            err
        })?;
    Ok(())
}

//=====================



#[derive(Deserialize, Validate, ToSchema)]
pub struct UpdateValidationSchemaRequestName {
    #[serde(rename = "name")]
    #[validate(length(min = 1, max = 32))]
    pub name: String,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct UpdateValidationSchemaRequestJson {
    #[serde(rename = "schema")]
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
    #[serde(rename = "schema")]
    pub json_schema: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}
impl From<PipelineValidationSchemaModel> for ValidationSchemaResponse {
    fn from(model: PipelineValidationSchemaModel) -> Self {
        ValidationSchemaResponse {
            id: model.id(),
            name: model.name().into(),
            json_schema: model.schema().into(),
            created_at: model.created_at().clone(),
            updated_at: model.updated_at().clone(),
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct ValidationSchemaByIdResponse {
    pub name: String,
    #[serde(rename = "schema")]
    pub json_schema: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}
impl From<PipelineNewValidateSchema> for ValidationSchemaByIdResponse {
    fn from(model: PipelineNewValidateSchema) -> Self {
        ValidationSchemaByIdResponse {
            name: model.name().into(),
            json_schema: model.schema().into(),
            created_at: model.created_at().clone(),
            updated_at: model.updated_at().clone(),
        }
    }
}
