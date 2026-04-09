use actix_web::{HttpResponse, ResponseError};
use crate::domain::error::{IoTBeeError, PipelinePersistenceError};
use actix_web::http::StatusCode;
use serde::Serialize;

#[derive(Serialize, utoipa::ToSchema)]
pub struct ErrorResponse {
    error: String,
}


impl ResponseError for PipelinePersistenceError {
    fn status_code(&self) -> StatusCode {
        match self {
            PipelinePersistenceError::ValidationSchemaNameExists { .. } => StatusCode::CONFLICT, // 409
            PipelinePersistenceError::ValidationSchemaNotFound { .. } => StatusCode::NOT_FOUND, // 404
                PipelinePersistenceError::Database { .. } => StatusCode::INTERNAL_SERVER_ERROR, // 500
                PipelinePersistenceError::SaveFailed { .. }
                | PipelinePersistenceError::UpdateFailed { .. }
                | PipelinePersistenceError::InvalidData { .. }
                | PipelinePersistenceError::DeleteFailed { .. } => StatusCode::BAD_REQUEST,
                PipelinePersistenceError::ParseError { .. } => StatusCode::BAD_REQUEST,
            }
            
        }
    

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        let body = match self {
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
            };

        HttpResponse::build(status).json(ErrorResponse { error: body })
    }

}





impl ResponseError for IoTBeeError {
    fn status_code(&self) -> StatusCode {
        match self {
            IoTBeeError::PipelinePersistenceError(inner) => inner.status_code(),
            IoTBeeError::PipelineError(_) => StatusCode::BAD_REQUEST,
            IoTBeeError::PipelineLifecycleError(_) => StatusCode::BAD_REQUEST,
            IoTBeeError::DataSourceError(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            IoTBeeError::PipelinePersistenceError(inner) => inner.error_response(),
            IoTBeeError::PipelineError(e) => {
                HttpResponse::build(StatusCode::BAD_REQUEST)
                    .json(ErrorResponse { error: format!("Pipeline error: {}", e) })
            }
            IoTBeeError::PipelineLifecycleError(e) => {
                HttpResponse::build(StatusCode::BAD_REQUEST)
                    .json(ErrorResponse { error: format!("Lifecycle error: {}", e) })
            }
            IoTBeeError::DataSourceError(e) => {
                HttpResponse::build(StatusCode::BAD_REQUEST)
                    .json(ErrorResponse { error: format!("Data source error: {}", e) })
            }
        }
    }
}


