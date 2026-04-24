use crate::domain::error::IoTBeeError;
use crate::domain::value_objects::pipelines_values::{DataStoreId, PipelineStatus};
use async_trait::async_trait;

#[async_trait]
pub trait PipelineLifecycle {
    async fn start(&self, pipeline_id: &DataStoreId) -> Result<(), IoTBeeError>;
    // async fn stop(&self, pipeline_id: &DataStoreId) -> Result<(), IoTBeeError>;
    // async fn get_status_by_id(
    //     &self,
    //     pipeline_id: &DataStoreId,
    // ) -> Result<PipelineStatus, IoTBeeError>;
    // async fn get_status(&self) -> Result<Vec<(DataStoreId, PipelineStatus)>, IoTBeeError>;
}
