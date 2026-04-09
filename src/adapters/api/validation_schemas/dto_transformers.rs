
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