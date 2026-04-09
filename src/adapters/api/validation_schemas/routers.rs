use actix_web::{HttpResponse, delete, get, post, put, web};
use crate::application::validation_schemas_cases::cases::SchemaValidationUseCases;
use super::models::{CreateValidationSchemaRequest, UpdateValidationSchemaRequest};
use crate::domain::error::{IoTBeeError,PipelinePersistenceError};
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


#[post("/create")]
async fn create_validation_schema(
    use_case: web::Data<UseCase>,
    body: web::Json<CreateValidationSchemaRequest>,
) -> Result<HttpResponse, IoTBeeError> {
    let schema_data: CreateValidationSchemaRequest = body.into_inner(); 

    schema_data.validate_values()?;
    
    use_case.create_validation_schema(schema_data.into()).await?;
    Ok(HttpResponse::Created().finish())    
}

#[get("/{id}")]
async fn get_validation_schema(
    _use_case: web::Data<UseCase>,
    _path: web::Path<i32>,
) -> HttpResponse {
    // TODO: llamar use_case.get_pipeline_validation_schema()
    HttpResponse::Ok().finish()
}

#[get("")]
async fn list_validation_schemas(
    _use_case: web::Data<UseCase>,
) -> HttpResponse {
    // TODO: llamar use_case.list_pipeline_validation_schema()
    HttpResponse::Ok().finish()
}

#[put("/{id}")]
async fn update_validation_schema(
    _use_case: web::Data<UseCase>,
    _path: web::Path<i32>,
    _body: web::Json<UpdateValidationSchemaRequest>,
) -> HttpResponse {
    // TODO: convertir _body a PipelineNewValidateSchema, llamar use_case.update_pipeline_validation_schema()
    HttpResponse::Ok().finish()
}

#[delete("/{id}")]
async fn delete_validation_schema(
    _use_case: web::Data<UseCase>,
    _path: web::Path<i32>,
) -> HttpResponse {
    // TODO: llamar use_case.delete_pipeline_validation_schema()
    HttpResponse::NoContent().finish()
}
