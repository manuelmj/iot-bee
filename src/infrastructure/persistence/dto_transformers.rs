

use crate::domain::entities::pipeline::{PipelineNewValidateSchema,PipelineValidationSchemaModel};
use crate::domain::value_objects::pipelines_values::{PipelineSchemaModel,DataStroreId};
use crate::domain::error::{IoTBeeError, PipelinePersistenceError};
use crate::infrastructure::persistence::models::{ValidationSchemaRow,ValidationSchemaRowWhitId};
use chrono::DateTime;

use std::convert::TryFrom;
use chrono::{Utc};

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


        let schema = PipelineSchemaModel::new(row.json_schema)?;

        Ok(PipelineNewValidateSchema::existing(
            row.json_name,
            schema,
            created_at,
            updated_at,
        ))
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

