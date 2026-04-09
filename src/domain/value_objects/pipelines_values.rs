
use crate::domain::error::IoTBeeError;


pub struct DataStroreId(i32);
impl DataStroreId {
    pub fn new(id: i32) -> Self {
        DataStroreId(id)
    }
    pub fn id(&self) -> i32 {
        self.0
    }
}

pub enum PipelineStatus{
    Running, 
    Stopped,
    Failed,
}

// Value objects relates to pipeline data source 
pub enum ConnectionType{
    RABBITMQ,
    KAFKA,
    MQTT,
}

pub struct PipelineDataSourceId(String);
pub struct PipilineDataSourceConnection(String);



pub struct PipelineConfig {
    // Define your pipeline configuration fields here
    data_base_source: String,
    data_source : String,
    processing_rules: Vec<String>,
    
}



// value objects relates to validation schemas 

pub struct PipelineSchemaModel(String);
impl PipelineSchemaModel {
    pub fn new(schema: String) -> Result<Self, IoTBeeError> {
        Ok(PipelineSchemaModel(schema))
    }
    pub fn schema(&self) -> &String {
        &self.0
    }
}

