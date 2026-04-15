

use crate::domain::error::{IoTBeeError,PipelinePersistenceError};
use crate::domain::entities::data_store::{PipelineDataStoreInputModel, PipelineDataStoreOutputModel};
use crate::domain::value_objects::pipelines_values::DataStroreId;
use crate::domain::outbound::pipeline_persistence::PipelineDataStoreRepository;
use crate::infrastructure::persistence::models::DataStoreRow;
use crate::infrastructure::persistence::connection::InternalDataBase;
use async_trait::async_trait;
use sqlx::Error as SqlxError;
use chrono::Utc;
use std::sync::Arc;

pub struct DataStoreRepository {
    pipeline_store_repository: Arc<InternalDataBase>,
}
impl DataStoreRepository{
    pub fn new(pipeline_store_repository: Arc<InternalDataBase>) -> Self {
        Self {
            pipeline_store_repository,
        }
    }
    pub fn data_base_connection(&self) -> &InternalDataBase {
        &self.pipeline_store_repository
    }
}

#[async_trait]
impl PipelineDataStoreRepository for DataStoreRepository {
    async fn save_pipeline_data_store(&self, data_store: &PipelineDataStoreInputModel) -> Result<(), IoTBeeError> {
        let pool = self.data_base_connection().pool(); 
        sqlx::query(
            r#"
            INSERT INTO databases (name, type, json_schema, description, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(data_store.name())
        .bind(data_store.type_id())
        .bind(data_store.configuration())
        .bind(data_store.data_store_description())
        .bind(Utc::now().to_rfc3339())
        .bind(Utc::now().to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| {
            match e {
                SqlxError::Database(db_error) if db_error.is_unique_violation() => {PipelinePersistenceError::ValidationSchemaNameExists { name: data_store.name().to_string() }},
                SqlxError::Database(db_error) if db_error.is_foreign_key_violation() => {PipelinePersistenceError::InvalidData { reason: db_error.to_string() }},
                _ => PipelinePersistenceError::SaveFailed { reason: e.to_string() },
            }
        })?;

        Ok(())
    }

    async fn get_pipeline_data_store(&self) -> Result<Vec<PipelineDataStoreOutputModel>, IoTBeeError> {
        let pool = self.data_base_connection().pool();
        println!("Fetching data stores from database...");
        let rows_result  = sqlx::query_as::<_, DataStoreRow>(
            r#"
            SELECT id, name, type, json_schema, description, created_at, updated_at
            FROM databases
            "#,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| 
            IoTBeeError::from(PipelinePersistenceError::Database{ reason: e.to_string() }))?;
        
        
        let result = rows_result.into_iter()
            .map(|row| row.try_into())
            .collect::<Result<Vec<PipelineDataStoreOutputModel>, IoTBeeError>>()?;
        
        println!("Retrieved data stores: {:?}", result.iter().map(|ds| ds.name()).collect::<Vec<_>>());
        Ok(result)
    }
    async fn get_pipeline_data_store_by_id(&self, data_store_id: &DataStroreId) -> Result<Option<PipelineDataStoreOutputModel>, IoTBeeError> {
        let pool = self.data_base_connection().pool();
        let row = sqlx::query_as::<_, DataStoreRow>(
            r#"
            SELECT id, name, type, json_schema, description, created_at, updated_at
            FROM databases
            WHERE id = ?
            "#,
        )
        .bind(data_store_id.id())
        .fetch_optional(pool)
        .await
        .map_err(|e| 
            IoTBeeError::from(PipelinePersistenceError::Database{ reason: e.to_string() }))?;

        if let Some(row) = row {
            let data_store: PipelineDataStoreOutputModel = row.try_into()?;
            Ok(Some(data_store))
        } else {
            Ok(None)
        }
    }
}