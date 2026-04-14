use crate::domain::entities::pipeline_groups::{PipelineGroupInputModel, PipelineGroupOutputModel};
use crate::domain::error::{IoTBeeError, PipelinePersistenceError};
use crate::domain::outbound::pipeline_persistence::PipelineGroupRepository;
use crate::domain::value_objects::pipelines_values::DataStroreId;
use crate::infrastructure::persistence::models::PipelineGroupRow;
use crate::infrastructure::persistence::repositories::pipeline_repository::PipelineStoreRepository;

use async_trait::async_trait;
use chrono::Utc;
use sqlx::Error as SqlxError;

#[async_trait]
impl PipelineGroupRepository for PipelineStoreRepository {
    async fn get_pipeline_group(&self) -> Result<Vec<PipelineGroupOutputModel>, IoTBeeError> {
        let pool = self.data_base_connection().pool();
        let result: Vec<PipelineGroupRow> = sqlx::query_as::<_, PipelineGroupRow>(
            r#"
            SELECT id, name, description, created_at, updated_at FROM pipeline_groups
            "#,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| PipelinePersistenceError::Database {
            reason: e.to_string(),
        })?;

        let output: Vec<PipelineGroupOutputModel> = result
            .into_iter()
            .map(|row| row.try_into())
            .collect::<Result<_, _>>()?;

        Ok(output)
    }

    async fn get_pipeline_group_by_id(
        &self,
        group_id: &DataStroreId,
    ) -> Result<Option<PipelineGroupOutputModel>, IoTBeeError> {
        let pool = self.data_base_connection().pool();
        let result: Option<PipelineGroupRow> = sqlx::query_as::<_, PipelineGroupRow>(
            r#"
            SELECT id, name, description, created_at, updated_at FROM pipeline_groups WHERE id = ?
            "#,
        )
        .bind(group_id.id())
        .fetch_optional(pool)
        .await
        .map_err(|e| PipelinePersistenceError::Database {
            reason: e.to_string(),
        })?;

        let output = result.map(|row| row.try_into()).transpose()?;
        Ok(output)
    }

    async fn save_pipeline_group(
        &self,
        group: &PipelineGroupInputModel,
    ) -> Result<(), IoTBeeError> {
        let pool = self.data_base_connection().pool();
        let result = sqlx::query(
            r#"
            INSERT INTO pipeline_groups (name, description, created_at, updated_at) VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(&group.name())
        .bind(&group.description())
        .bind(Utc::now().to_rfc3339())
        .bind(Utc::now().to_rfc3339())
        .execute(pool)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(SqlxError::Database(db_error)) if db_error.is_unique_violation() => {
                Err(PipelinePersistenceError::ValidationSchemaNameExists {
                    name: group.name().to_string(),
                }
                .into())
            }
            Err(e) => Err(IoTBeeError::from(PipelinePersistenceError::SaveFailed {
                reason: e.to_string(),
            })),
        }
    }
}
