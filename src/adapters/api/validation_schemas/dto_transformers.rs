
use std::convert::From; 
use crate::application::validation_schemas_cases::validation_entities::ValidationSchema;
use crate::adapters::api::validation_schemas::models::CreateValidationSchemaRequest; 


impl From<CreateValidationSchemaRequest> for ValidationSchema {
    fn from(request: CreateValidationSchemaRequest) -> Self {
        ValidationSchema {
            name: request.name,
            schema: request.json_schema,
        }
    }
}


use crate::application::validation_schemas_cases::validation_entities::{ValidationSchemaModel};
use crate::adapters::api::validation_schemas::models::{ValidationSchemaResponse};



impl From<ValidationSchemaModel> for ValidationSchemaResponse {
    fn from(model: ValidationSchemaModel) -> Self {
        ValidationSchemaResponse {
            id: model.id,
            name: model.name,
            json_schema: model.schema,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}


use crate::application::validation_schemas_cases::validation_entities::{ValidationSchemeModeById};
use crate::adapters::api::validation_schemas::models::{ValidationSchemaByIdResponse};

impl From<ValidationSchemeModeById> for ValidationSchemaByIdResponse {
    fn from(model: ValidationSchemeModeById) -> Self {
        ValidationSchemaByIdResponse {
            name: model.name,
            json_schema: model.schema,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}