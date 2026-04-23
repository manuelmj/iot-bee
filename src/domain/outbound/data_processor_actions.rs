use async_trait::async_trait;
use crate::domain::error::IoTBeeError;  
use crate::domain::entities::validation_schema::PipelineValidationSchemaModel;
use crate::domain::entities::data_consumer_types::DataConsumerRawType;

#[async_trait]
pub trait DataProcessorActions {
    async fn process_data(&self,data_to_process: DataConsumerRawType, process_rules: PipelineValidationSchemaModel) -> Result<(), IoTBeeError>;
}
