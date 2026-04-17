use crate::domain::error::IoTBeeError;
use crate::domain::value_objects::pipelines_values::DataStoreId;
use async_trait::async_trait;


#[async_trait]
pub trait PipelineLifecycle {
    async fn start(&self, pipeline_id: &DataStoreId) -> Result<(), IoTBeeError>;
    async fn stop(&self, pipeline_id: &DataStoreId) -> Result<(), IoTBeeError>;
}
