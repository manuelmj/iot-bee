use async_trait::async_trait;
use crate::domain::outbound::PipelineGeneralRepository;
use crate::domain::outbound::pipeline_persistence::PipelineValidationSchemaRepository;
use crate::domain::error::{IoTBeeError, PipelinePersistenceError};
use crate::application::validation_schemas_cases::validation_entities::ValidationSchema;

use crate::domain::entities::pipeline::{
    PipelineNewValidateSchema
}; 

use crate::domain::value_objects::pipelines_values::PipelineSchemaModel;


#[async_trait]
pub trait SchemaValidationUseCases{
    // reglas de negocio para la validacion de esquemas de pipeline

    // se puede crear un nuevo esquema de validacion siempre y cuando el nombre del esquema no exista ya en la base de datos.
    async fn create_validation_schema(&self,eschemas_model : ValidationSchema) -> Result<(), IoTBeeError>;
    // no se puede eliminar un esquema de validacion si este esta siendo utilizado por algun pipeline en la base de datos.
    // async fn delete_validation_schema(&self) -> Result<(), IoTBeeError>;
    // se puede actualizar un esquema de validacion siempre y cuando el nuevo nombre del esquema no exista ya en la base de datos.
    // siempre que se actualice un esquema de validacion se deben reiniciar los pipelines que lo esten utilizando.
    // async fn update_validation_schema(&self) -> Result<(), IoTBeeError>;
    // se puede obtener un esquema de validacion por su id.
    // async fn get_validation_schema(&self);
    // se pueden listar todos los esquemas de validacion disponibles en la base de datos.
    // async fn list_validation_schemas(&self);
}




pub struct SchemaValidationUseCasesImpl<T: PipelineGeneralRepository + Send + Sync>{
    repository: T,
}

impl<T: PipelineGeneralRepository + Send + Sync> SchemaValidationUseCasesImpl<T>{
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

}

#[async_trait]
impl<T> SchemaValidationUseCases for SchemaValidationUseCasesImpl<T>
where T: PipelineGeneralRepository + Send + Sync
{

    async fn create_validation_schema(&self, eschemas_model: ValidationSchema) -> Result<(), IoTBeeError> {
        
        //para crear un nuevo esquema de validacion 
        // 1. convertir el ValidationSchema a PipelineNewValidateSchema
        // 2. validar si existe otro esquema de validacion con el mismo nombre en la base de datos llamando al metodo list_pipeline_validation_schema_names del repositorio
        // 3. llamar al metodo save_pipeline_validation_schema del repositorio para guardar el nuevo
        // 4. devolver el resultado.

        let schema = PipelineSchemaModel::new(eschemas_model.schema)?;
        let domain_schema = PipelineNewValidateSchema::new(
            eschemas_model.name,
            schema
        );

        
        // if self.repository.pipeline_validation_schema_exists_name(domain_schema.name()).await? {
            // return Err(PipelinePersistenceError::ValidationSchemaNameExists {
                // name: domain_schema.name().to_string(),
            // }.into());
        // }

        self.repository.save_pipeline_validation_schema(&domain_schema).await?;


        Ok(())
    }

}




