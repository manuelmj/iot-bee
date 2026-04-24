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



use super::general_messages::{SendActorActionMessageResult};
#[async_trait]
pub trait SendActionToActor: Send + Sync {
    // async fn send_start_actor(&self) -> SendActorActionMessageResult;
    async fn send_stop_actor(&self) -> SendActorActionMessageResult;
    async fn send_restart_actor(&self) -> SendActorActionMessageResult;
    async fn get_actor_status(&self) -> SendActorActionMessageResult;
    
}