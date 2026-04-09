use actix_web::{HttpResponse, ResponseError};
use actix_web::http::StatusCode;
use crate::domain::error::{IoTBeeError, PipelinePersistenceError};
use serde::Serialize;

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl ResponseError for IoTBeeError {
    fn status_code(&self) -> StatusCode {
        match self {
            IoTBeeError::PipelinePersistenceError(inner) => match inner {
                PipelinePersistenceError::ValidationSchemaNameExists { .. } => StatusCode::CONFLICT, // 409
                PipelinePersistenceError::ValidationSchemaNotFound { .. } => StatusCode::NOT_FOUND, // 404
                PipelinePersistenceError::Database { .. } => StatusCode::INTERNAL_SERVER_ERROR,      // 500
                PipelinePersistenceError::SaveFailed { .. }
                | PipelinePersistenceError::UpdateFailed { .. }
                | PipelinePersistenceError::InvalidData { .. }
                | PipelinePersistenceError::DeleteFailed { .. } => StatusCode::BAD_REQUEST,
                PipelinePersistenceError::ParseError { .. } => StatusCode::BAD_REQUEST,
            },
            IoTBeeError::PipelineError(_) | IoTBeeError::PipelineLifecycleError(_) | IoTBeeError::DataSourceError(_) => {
                StatusCode::BAD_REQUEST
            }
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        let body = match self {
            IoTBeeError::PipelinePersistenceError(inner) => match inner {
                PipelinePersistenceError::ValidationSchemaNameExists { name } => {
                    format!("Validation schema with name '{}' already exists", name)
                }
                PipelinePersistenceError::ValidationSchemaNotFound { schema_id } => {
                    format!("Validation schema with id '{}' not found", schema_id)
                }
                PipelinePersistenceError::Database { .. } => "Internal server error".to_string(),
                PipelinePersistenceError::SaveFailed { reason }
                | PipelinePersistenceError::UpdateFailed { reason }
                | PipelinePersistenceError::DeleteFailed { reason }
                | PipelinePersistenceError::ParseError { reason }
                | PipelinePersistenceError::InvalidData { reason } => reason.clone(),
            },
            IoTBeeError::PipelineError(inner) => inner.to_string(),
            IoTBeeError::PipelineLifecycleError(inner) => inner.to_string(),
            IoTBeeError::DataSourceError(inner) => inner.to_string(),
        };

        HttpResponse::build(status).json(ErrorResponse { error: body })
    }
}



