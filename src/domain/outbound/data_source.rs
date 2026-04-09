use async_trait::async_trait;

use crate::domain::error::DataSourceError;

// this port describe  how the sistem consume data from external queue data sources, such as MQTT, Kafka, Rabbitmq, etc.
//

#[async_trait]
pub trait DataSource {
    async fn receive(&self) -> Result<String, DataSourceError>;
    async fn ack(&self, message_id: &str) -> Result<(), DataSourceError>;
}
