use thiserror::Error;

pub type DomainResult<T> = Result<T, IoTBeeError>;

#[derive(Error, Debug)]
pub enum PipelineError {
    #[error("Pipeline name is invalid")]
    InvalidName,
    #[error("Pipeline configuration is invalid")]
    InvalidConfig,
    #[error("Pipeline with id {pipeline_id} does not exist")]
    NotFound { pipeline_id: String },
}

#[derive(Error, Debug)]
pub enum PipelineLifecycleError {
    #[error("Pipeline with id {pipeline_id} is already running")]
    AlreadyRunning { pipeline_id: String },
    #[error("Pipeline with id {pipeline_id} is already stopped")]
    AlreadyStopped { pipeline_id: String },
    #[error("Lifecycle operation failed: {reason}")]
    OperationFailed { reason: String },
}

#[derive(Error, Debug)]
pub enum DataSourceError {
    #[error("Data source connection failed: {reason}")]
    ConnectionFailed { reason: String },
    #[error("Data source timeout")]
    Timeout,
    #[error("Could not decode payload: {reason}")]
    InvalidPayload { reason: String },
}

#[derive(Error, Debug)]
pub enum PipelinePersistenceError {
    #[error("Pipeline could not be persisted: {reason}")]
    SaveFailed { reason: String },
    #[error("Pipeline could not be deleted: {reason}")]
    UpdateFailed { reason: String },
    #[error("Pipeline could not be updated: {reason}")]
    DeleteFailed { reason: String },
    #[error("Database operation failed: {reason}")]
    Database { reason: String },
    #[error("Failed to parse data: {reason}")]
    ParseError { reason: String },
    #[error("Pipeline validation schema with name {name} already exists")]
    ValidationSchemaNameExists { name: String },
    #[error("Invalid data for pipeline validation schema: {reason}")]
    InvalidData { reason: String },
    #[error("Pipeline validation schema with id {schema_id} not found")]
    ValidationSchemaNotFound { schema_id: String },
}

// define a proper domain error for all my sistem 
#[derive(Error, Debug)]
pub enum IoTBeeError {
    #[error("Pipeline error: {0}")]
    PipelineError(#[from] PipelineError),
    #[error("Pipeline lifecycle error: {0}")]
    PipelineLifecycleError(#[from] PipelineLifecycleError),
    #[error("Data source error: {0}")]
    DataSourceError(#[from] DataSourceError),
    #[error("Persistence error: {0}")]
    PipelinePersistenceError(#[from] PipelinePersistenceError),
}
