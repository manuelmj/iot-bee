use crate::domain::outbound::pipeline_persistence::PipelineConnectionTypeRepository;
// use crate::domain::outbound::PipelineGeneralRepository;
use crate::domain::entities::pipeline::ConnectionTypeModel;
use crate::domain::error::{IoTBeeError, PipelinePersistenceError};
use crate::infrastructure::persistence::repositories::pipeline_repository::PipelineStoreRepository;
use async_trait::async_trait;

use crate::infrastructure::persistence::models::ConnectionTypeRow;
// use sqlx::Error as SqlxError;

#[async_trait]
impl PipelineConnectionTypeRepository for PipelineStoreRepository {
    async fn get_pipeline_connection_type(&self) -> Result<Vec<ConnectionTypeModel>, IoTBeeError> {
        // Implementation to get the pipeline connection type from the database
        let pool = self.data_base_connection().pool();
        let result = sqlx::query_as::<_, ConnectionTypeRow>(
            r#"
            SELECT id, connection_type FROM connection_types
            "#  ,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| PipelinePersistenceError::Database {
            reason: e.to_string(),
        })?;

        let connection_types: Vec<ConnectionTypeModel> = result
            .into_iter()
            .map(|row| ConnectionTypeModel::try_from(row))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(connection_types)
    }
}
