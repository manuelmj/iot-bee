// use async_trait::async_trait;

// use crate::domain::error::IoTBeeError;
// use crate::domain::value_objects::pipelines_values::{PipelineId, PipelineStatus};


// #[async_trait]
// pub trait PipelineManager {
//     async fn start_pipeline(&self, pipeline_id: &PipelineId) -> Result<(), IoTBeeError>;
//     async fn stop_pipeline(&self, pipeline_id: &PipelineId) -> Result<(), IoTBeeError>;
//     async fn update_pipeline(&self, pipeline_id: &PipelineId) -> Result<(), IoTBeeError>;
//     async fn get_pipeline_status(&self, pipeline_id: &PipelineId) -> Result<PipelineStatus, IoTBeeError>;
// }