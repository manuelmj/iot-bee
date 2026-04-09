use crate::adapters::api::validation_schemas::models::CreateValidationSchemaRequest;
use crate::application::validation_schemas_cases::validation_entities::ValidationSchema;
use std::convert::From;

impl From<CreateValidationSchemaRequest> for ValidationSchema {
    fn from(request: CreateValidationSchemaRequest) -> Self {
        ValidationSchema {
            name: request.name,
            schema: request.json_schema,
        }
    }
}

use crate::adapters::api::validation_schemas::models::ValidationSchemaResponse;
use crate::application::validation_schemas_cases::validation_entities::ValidationSchemaModel;

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

use crate::adapters::api::validation_schemas::models::ValidationSchemaByIdResponse;
use crate::application::validation_schemas_cases::validation_entities::ValidationSchemeModeById;

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
