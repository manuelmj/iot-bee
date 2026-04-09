mod composition;

use actix_web::{App, HttpServer, web};
use iot_bee::adapters::api::validation_schemas::routers::validation_schemas_scope;
use iot_bee::adapters::api::connection_types::routers::connection_types_scope;
use iot_bee::infrastructure::persistence::connection::SqliteDb;
use iot_bee::infrastructure::persistence::repositories::pipeline_repository::PipelineStoreRepository;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use iot_bee::application::validation_schemas_cases::cases::{
    SchemaValidationUseCases, SchemaValidationUseCasesImpl,
};

use iot_bee::adapters::api::error::ErrorResponse;
use iot_bee::adapters::api::validation_schemas::models::{
    CreateValidationSchemaRequest, UpdateValidationSchemaRequestJson,
    UpdateValidationSchemaRequestName, ValidationSchemaByIdResponse, ValidationSchemaResponse,
};
use iot_bee::adapters::api::validation_schemas::routers as validation_routers;
use iot_bee::adapters::api::connection_types::routers as connection_routers;

//para los casos de uso 
use iot_bee::application::connection_types_cases::cases::ConnectionTypesUseCasesImpl;
use iot_bee::application::connection_types_cases::cases::ConnectionTypesUseCases;

#[derive(OpenApi)]
#[openapi(
    paths(
        validation_routers::create_validation_schema,
        validation_routers::get_validation_schema,
        validation_routers::list_validation_schemas,
        validation_routers::update_validation_schema,
        validation_routers::update_validation_schema_json,
        validation_routers::delete_validation_schema,
    ),
    components(
        schemas(
            CreateValidationSchemaRequest,
            UpdateValidationSchemaRequestName,
            UpdateValidationSchemaRequestJson,
            ValidationSchemaResponse,
            ValidationSchemaByIdResponse,
            ErrorResponse,
        )
    ),
    tags(
        (name = "Validation Schemas", description = "CRUD operations for pipeline validation schemas")
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    let db = SqliteDb::new(&database_url)
        .await
        .expect("Failed to connect to database");

    //implementacion concreta del repositorio de validaciones de pipeline utilizando la conexion a la base de datos
    // let repo: PipelineStoreRepository = PipelineStoreRepository::new(db);
    let repo = Arc::new(PipelineStoreRepository::new(db));   
    //uso de caso al que se inyecta el repo
    let validation_schemas_use_case: Arc<dyn SchemaValidationUseCases + Send + Sync> =
        Arc::new(SchemaValidationUseCasesImpl::new(repo.clone()));
    let validation_schemas_use_case_data = web::Data::from(validation_schemas_use_case.clone());
        
    //casos de uso del connection types 
    let connection_types_use_case: Arc<dyn ConnectionTypesUseCases + Send + Sync> =
        Arc::new(ConnectionTypesUseCasesImpl::new(repo.clone()));
    let connection_types_use_case_data = web::Data::from(connection_types_use_case.clone());

        
    println!("IoT Bee is starting on http://127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .service(validation_schemas_scope(validation_schemas_use_case_data.clone()))
            .service(connection_types_scope(connection_types_use_case_data.clone()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
