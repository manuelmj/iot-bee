use crate::domain::error::IoTBeeError;

pub struct DataStroreId(u32);
impl DataStroreId {
    pub fn new(id: u32) -> Self {
        DataStroreId(id)
    }
    pub fn id(&self) -> u32 {
        self.0
    }
}

pub enum PipelineStatus {
    Running,
    Stopped,
    Failed,
}

// Value objects relates to pipeline data source
pub enum ConnectionType {
    RABBITMQ,
    KAFKA,
    MQTT,
}

pub struct PipelineDataSourceId(String);
pub struct PipilineDataSourceConnection(String);

pub struct PipelineConfig {
    // Define your pipeline configuration fields here
    data_base_source: String,
    data_source: String,
    processing_rules: Vec<String>,
}

// value objects relates to validation schemas

pub struct PipelineSchemaModel(String);
impl PipelineSchemaModel {
    pub fn new(schema: impl Into<String>) -> Result<Self, IoTBeeError> {
        //validar aca las caracteristicas que debe tener el schema, por ejemplo que sea un json valido, o que tenga ciertas propiedades, etc.
        // devolver el error de validacion de datos.
        Ok(PipelineSchemaModel(schema.into()))
    }
    pub fn schema(&self) -> &str {
        &self.0
    }
}
