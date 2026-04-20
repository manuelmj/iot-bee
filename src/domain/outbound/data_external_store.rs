use async_trait::async_trait;
use crate::domain::entities::data_consumer_types::DataConsumerRawType;
use crate::domain::error::IoTBeeError;
#[async_trait]
pub trait DataExternalStore {
    async fn save(&self, data: DataConsumerRawType) -> Result<(), IoTBeeError>;
}
