use crate::domain::entities::pipeline::{
    ConnectionTypeModel, PipelineNewValidateSchema, PipelineValidationSchemaModel,
};
use crate::domain::error::{IoTBeeError, PipelinePersistenceError};
use crate::domain::value_objects::pipelines_values::{DataStroreId, PipelineSchemaModel};
use crate::infrastructure::persistence::models::{
    ConnectionTypeRow, ValidationSchemaRow, ValidationSchemaRowWhitId,
};

use chrono::DateTime;

use chrono::Utc;
use std::convert::TryFrom;

impl TryFrom<ValidationSchemaRow> for PipelineNewValidateSchema {
    type Error = IoTBeeError;

    fn try_from(row: ValidationSchemaRow) -> Result<Self, Self::Error> {
        let created_at = DateTime::parse_from_rfc3339(&row.created_at)
            .map_err(|e| PipelinePersistenceError::Database {
                reason: format!("invalid created_at: {}", e),
            })?
            .with_timezone(&Utc);

        let updated_at = DateTime::parse_from_rfc3339(&row.updated_at)
            .map_err(|e| PipelinePersistenceError::Database {
                reason: format!("invalid updated_at: {}", e),
            })?
            .with_timezone(&Utc);

        let result = PipelineNewValidateSchema::existing(
            row.json_name,
            row.json_schema,
            created_at,
            updated_at,
        )?;

        Ok(result)
    }
}

impl TryFrom<ValidationSchemaRowWhitId> for PipelineValidationSchemaModel {
    type Error = IoTBeeError;

    fn try_from(row: ValidationSchemaRowWhitId) -> Result<Self, Self::Error> {
        let created_at = DateTime::parse_from_rfc3339(&row.created_at)
            .map_err(|e| PipelinePersistenceError::Database {
                reason: format!("invalid created_at: {}", e),
            })?
            .with_timezone(&Utc);

        let updated_at = DateTime::parse_from_rfc3339(&row.updated_at)
            .map_err(|e| PipelinePersistenceError::Database {
                reason: format!("invalid updated_at: {}", e),
            })?
            .with_timezone(&Utc);

        let schema = PipelineSchemaModel::new(row.json_schema)?;

        Ok(PipelineValidationSchemaModel::new(
            DataStroreId::new(row.id),
            row.json_name,
            schema,
            created_at,
            updated_at,
        ))
    }
}

impl TryFrom<ConnectionTypeRow> for ConnectionTypeModel {
    type Error = IoTBeeError;

    fn try_from(row: ConnectionTypeRow) -> Result<Self, Self::Error> {
        Ok(ConnectionTypeModel::new(row.connection_type, row.id)?)
    }
}
