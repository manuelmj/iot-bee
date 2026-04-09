
use async_trait::async_trait;
use crate::domain::outbound::PipelineGeneralRepository;
use crate::domain::outbound::pipeline_persistence::PipelineValidationSchemaRepository;
use crate::domain::error::{IoTBeeError, PipelinePersistenceError};

use crate::domain::entities::pipeline::{
    PipelineNewValidateSchema
}; 

use crate::domain::value_objects::pipelines_values::PipelineSchemaModel;
use crate::domain::value_objects::pipelines_values::DataStroreId;

use crate::application::validation_schemas_cases::validation_entities::{ValidationSchema, ValidationSchemaModel, ValidationSchemeModeById};

#[async_trait]
pub trait SchemaValidationUseCases{
    // reglas de negocio para la validacion de esquemas de pipeline

    // se puede crear un nuevo esquema de validacion siempre y cuando el nombre del esquema no exista ya en la base de datos.
    async fn create_validation_schema(&self,eschemas_model : &ValidationSchema) -> Result<(), IoTBeeError>;
    
    // no se puede eliminar un esquema de validacion si este esta siendo utilizado por algun pipeline en la base de datos.
    // async fn delete_validation_schema(&self) -> Result<(), IoTBeeError>;
    
    // se puede actualizar el nombre de un esquema de validacion siempre y cuando el nuevo nombre del esquema no exista ya en la base de datos.
    // siempre que se actualice un esquema de validacion se deben reiniciar los pipelines que lo esten utilizando.
    async fn update_validation_schema(&self, schema_id: u32, new_schema: &str) -> Result<(), IoTBeeError>;
    async fn update_validation_schema_name(&self, schema_id:u32, new_name: &str) -> Result<(), IoTBeeError>;
    
    // se puede obtener un esquema de validacion por su id.
    async fn get_validation_schema(&self)-> Result<Vec<ValidationSchemaModel>, IoTBeeError>;
    
    // se puede obtener un esquema de validacion usando solo su id 
    async fn get_validation_schema_by_id(&self, id: u32) -> Result<Option<ValidationSchemeModeById>, IoTBeeError>;
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

    async fn create_validation_schema(&self, schema_model: &ValidationSchema) -> Result<(), IoTBeeError> {
        
        //para crear un nuevo esquema de validacion 
        // 1. convertir el ValidationSchema a PipelineNewValidateSchema
        // 2. validar si existe otro esquema de validacion con el mismo nombre en la base de datos llamando al metodo list_pipeline_validation_schema_names del repositorio
        // 3. llamar al metodo save_pipeline_validation_schema del repositorio para guardar el nuevo
        // 4. devolver el resultado.

        let domain_schema = PipelineNewValidateSchema::new(
            schema_model.name(),
            schema_model.schema()
        )?;

        self.repository.save_pipeline_validation_schema(&domain_schema).await?;

        Ok(())
    }
    async fn get_validation_schema(&self) -> Result<Vec<ValidationSchemaModel>, IoTBeeError> {
        // para obtener un esquema de validacion por su id
        // 1. llamar al metodo get_pipeline_validation_schema del repositorio para obtener el esquema de validacion
        // 2. convertir el PipelineValidationSchemaModel a ValidationSchemaModel
        // 3. devolver el resultado.

        let schemas = self.repository.list_pipeline_validation_schema().await?;

        let result: Vec<ValidationSchemaModel> = schemas.into_iter().map(|s| {
            ValidationSchemaModel::new(
                s.id(),
                s.name(),
                s.schema(),
                s.created_at().clone(),
                s.updated_at().clone()
            )
        }).collect();

        Ok(result)

    }

    async fn get_validation_schema_by_id(&self, id: u32) -> Result<Option<ValidationSchemeModeById>, IoTBeeError> {
        // para obtener un esquema de validacion por su id
        // 1. llamar al metodo get_pipeline_validation_schema_by_id del repositorio para obtener el esquema de validacion
        // 2. convertir el PipelineValidationSchemaModel a ValidationSchemeModeById
        // 3. devolver el resultado.
        
        let id_to_search: DataStroreId = DataStroreId::new(id);
        let schema: Option<PipelineNewValidateSchema> = self.repository.get_pipeline_validation_schema(&id_to_search).await?;

        match schema {
            Some(s) => Ok(Some(ValidationSchemeModeById::new(
                s.name().to_string(),
                s.schema().to_string(),
                s.created_at().clone(),
                s.updated_at().clone()
            ))),
            None => Ok(None)
        }

    }

    async fn update_validation_schema_name(&self, schema_id: u32, new_name: &str) -> Result<(), IoTBeeError> {
        // para actualizar el nombre de un esquema de validacion
        // 1. validar si existe un esquema de validacion con el id proporcionado en la base de datos llamando al metodo get_pipeline_validation_schema_by_id del repositorio
        // 2. validar si existe otro esquema de validacion con el mismo nombre en la
        // base de datos llamando al metodo list_pipeline_validation_schema_names del repositorio
        // 3. llamar al metodo update_pipeline_validation_schema_name del repositorio para actualizar el

        let id_to_search: DataStroreId = DataStroreId::new(schema_id);
        let result = self.repository.get_pipeline_validation_schema(&id_to_search).await?;
        if result.is_none() {
            return Err(
                IoTBeeError::from(
                    PipelinePersistenceError::ValidationSchemaNotFound { schema_id: schema_id.to_string() }
                )
            );
        }
        self.repository.update_pipeline_validation_schema_name(&id_to_search, new_name).await?;

        Ok(())
    }


    async fn update_validation_schema(&self, schema_id: u32, new_schema: &str) -> Result<(), IoTBeeError> {
        // para actualizar el esquema de validacion
        // 1. validar si existe un esquema de validacion con el id proporcionado en la base de datos llamando al metodo get_pipeline_validation_schema_by_id del repositorio
        // 2. llamar al metodo update_pipeline_validation_schema del repositorio para actualizar el esquema de validacion
        let id_to_search: DataStroreId = DataStroreId::new(schema_id);
        let result = self.repository.get_pipeline_validation_schema(&id_to_search).await?;
        if result.is_none() {
            return Err(
                IoTBeeError::from(
                    PipelinePersistenceError::ValidationSchemaNotFound { schema_id: schema_id.to_string() }
                )
            );
        }
        let new_schema = PipelineNewValidateSchema::new(
            "".to_string(), // El nombre no se actualizará en este método, así que se puede usar un valor temporal
            new_schema.to_string()
        )?;
        self.repository.update_pipeline_validation_schema(&id_to_search, &new_schema).await?;

        Ok(())
    }


}




