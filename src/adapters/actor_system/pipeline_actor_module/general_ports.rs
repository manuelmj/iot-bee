use crate::domain::entities::data_consumer_types::DataConsumerRawType;
use crate::domain::error::IoTBeeError;
use async_trait::async_trait;

#[async_trait]
pub trait SendDataToProcessor: Send + Sync {
    async fn send(&self, data: &DataConsumerRawType) -> Result<(), IoTBeeError>;
}

#[async_trait]
pub trait SendDataToStore: Send + Sync {
    async fn send(&self, data: &DataConsumerRawType) -> Result<(), IoTBeeError>;
}
