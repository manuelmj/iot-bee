use crate::domain::entities::pipeline::ConnectionTypeModel;
use crate::domain::error::IoTBeeError;
use crate::domain::outbound::PipelineGeneralRepository;
use std::sync::Arc;
use async_trait::async_trait;

pub struct ConnectionType {
    pub id: u32,
    pub connection_type: String,
}
impl ConnectionType {
    pub fn new(id: u32, connection_type: impl Into<String>) -> Self {
        Self {
            id,
            connection_type: connection_type.into(),
        }
    }
}

#[async_trait]
pub trait ConnectionTypesUseCases {
    async fn get_all_connection_types(&self) -> Result<Vec<ConnectionType>, IoTBeeError>;
}

pub struct ConnectionTypesUseCasesImpl<T: PipelineGeneralRepository + Send + Sync> {
    repository: Arc<T>,
}

impl<T: PipelineGeneralRepository + Send + Sync> ConnectionTypesUseCasesImpl<T> {
    pub fn new(repository: Arc<T>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<T: PipelineGeneralRepository + Send + Sync> ConnectionTypesUseCases
    for ConnectionTypesUseCasesImpl<T>
{
    async fn get_all_connection_types(&self) -> Result<Vec<ConnectionType>, IoTBeeError> {
        let connection_types_models: Vec<ConnectionTypeModel> =
            self.repository.get_pipeline_connection_type().await?;

        let connection_types: Vec<ConnectionType> = connection_types_models
            .into_iter()
            .map(|model| ConnectionType::new(model.id(), model.connection_type()))
            .collect();

        Ok(connection_types)
    }
}
