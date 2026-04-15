use crate::domain::entities::connection_type::ConnectionTypeModel;
use crate::domain::error::IoTBeeError;
use crate::domain::outbound::pipeline_persistence::PipelineConnectionTypeRepository;
use async_trait::async_trait;
use std::sync::Arc;

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

pub struct ConnectionTypesUseCasesImpl<T: PipelineConnectionTypeRepository + Send + Sync> {
    connection_type_repository: Arc<T>,
}

impl<T: PipelineConnectionTypeRepository + Send + Sync> ConnectionTypesUseCasesImpl<T> {
    pub fn new(connection_type_repository: Arc<T>) -> Self {
        Self { connection_type_repository }
    }
}

#[async_trait]
impl<T> ConnectionTypesUseCases for ConnectionTypesUseCasesImpl<T>
where
    T: PipelineConnectionTypeRepository + Send + Sync,
{
    async fn get_all_connection_types(&self) -> Result<Vec<ConnectionType>, IoTBeeError> {
        let connection_types_models: Vec<ConnectionTypeModel> =
            self.connection_type_repository.get_pipeline_connection_type().await?;

        let connection_types: Vec<ConnectionType> = connection_types_models
            .into_iter()
            .map(|model| ConnectionType::new(model.id(), model.connection_type()))
            .collect();

        Ok(connection_types)
    }
}
