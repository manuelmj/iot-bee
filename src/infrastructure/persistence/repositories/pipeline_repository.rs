use crate::infrastructure::persistence::connection::SqliteDb;
use crate::domain::outbound::PipelineGeneralRepository;
use async_trait::async_trait;
pub struct PipelineStoreRepository{
    data_base_connection: SqliteDb,
}

impl PipelineStoreRepository {
    pub fn new(data_base_connection: SqliteDb) -> Self {
        Self { data_base_connection }
    }

    pub fn data_base_connection(&self) -> &SqliteDb {
        &self.data_base_connection
    }

}


#[async_trait]
impl PipelineGeneralRepository for PipelineStoreRepository {}
