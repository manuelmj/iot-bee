use actix_web::{HttpResponse, delete, get, post, put, web};
use crate::application::validation_schemas_cases::cases::SchemaValidationUseCases;
use super::models::{CreateValidationSchemaRequest, UpdateValidationSchemaRequestJson, UpdateValidationSchemaRequestName};
use crate::domain::error::{IoTBeeError};
use crate::adapters::api::validation_schemas::models::{ValidationSchemaResponse, ValidationSchemaByIdResponse, SchemaId};
use crate::application::validation_schemas_cases::validation_entities::{ValidationSchemaModel};
use validator::Validate;

type UseCase = dyn SchemaValidationUseCases + Send + Sync;


pub fn validation_schemas_scope(use_case: web::Data<UseCase>) -> actix_web::Scope {
    web::scope("/validation-schemas")
        .app_data(use_case)
        .service(create_validation_schema)
        .service(list_validation_schemas)
        .service(get_validation_schema)
        .service(update_validation_schema)
        .service(delete_validation_schema)
}


#[post("")]
async fn create_validation_schema(
    use_case: web::Data<UseCase>,
    body: web::Json<CreateValidationSchemaRequest>,
) -> Result<HttpResponse, IoTBeeError> {
    
    let schema_data: CreateValidationSchemaRequest = body.into_inner(); 
    schema_data.validate_values()?;
    use_case.create_validation_schema(&schema_data.into()).await?;

    Ok(HttpResponse::Created().finish())    
}

#[get("/{id}")]
async fn get_validation_schema(
    use_case: web::Data<UseCase>,
    id_path: web::Path<SchemaId>,
) -> Result<HttpResponse, IoTBeeError> {
    let id = id_path.into_inner();
    let result = use_case.get_validation_schema_by_id(id).await?;

    match result {
        Some(schema) => {
            let response: ValidationSchemaByIdResponse = schema.into();
            Ok(HttpResponse::Ok().json(response))
        }
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

#[get("")]
async fn list_validation_schemas(
    _use_case: web::Data<UseCase>,
) -> Result<HttpResponse, IoTBeeError> {
    
    let result: Vec<ValidationSchemaModel> = _use_case.get_validation_schema().await?;
    let result: Vec<ValidationSchemaResponse> = result.into_iter().map(ValidationSchemaResponse::from)
    .collect();   
    
    Ok(HttpResponse::Ok().json(result))
}

#[put("/{id}/name")]
async fn update_validation_schema(
    use_case: web::Data<UseCase>,
    id_path: web::Path<i32>,
    body: web::Json<UpdateValidationSchemaRequestName>,
) ->Result<HttpResponse, IoTBeeError> {
    let id = id_path.into_inner();
    let schema_data: UpdateValidationSchemaRequestName = body.into_inner();
    schema_data.validate().map_err(|e| IoTBeeError::from(
        crate::domain::error::PipelinePersistenceError::InvalidData { reason: e.to_string() }
    ))?;

    use_case.update_validation_schema_name(id as u32, &schema_data.name).await?;

    Ok(HttpResponse::Ok().finish())
}

#[put("/{id}/schema")]
async fn update_validation_schema_json(
    use_case: web::Data<UseCase>,
    id_path: web::Path<i32>,
    body: web::Json<UpdateValidationSchemaRequestJson>,
) ->Result<HttpResponse, IoTBeeError> {
    let id = id_path.into_inner();
    let schema_data: UpdateValidationSchemaRequestJson = body.into_inner();
    schema_data.validate_values()?;
    use_case.update_validation_schema(id as u32, schema_data.json_schema()).await?;
    Ok(HttpResponse::Ok().finish())
}




#[delete("/{id}")]
async fn delete_validation_schema(
    _use_case: web::Data<UseCase>,
    _path: web::Path<i32>,
) -> Result<HttpResponse, IoTBeeError> {
    // TODO: llamar use_case.delete_pipeline_validation_schema()
    Ok(HttpResponse::NoContent().finish())
}
