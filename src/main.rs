mod composition;

use actix_web::{App, HttpServer, web};
use iot_bee::adapters::api::connection_types::routers::connection_types_scope;
use iot_bee::adapters::api::validation_schemas::routers::validation_schemas_scope;

use iot_bee::infrastructure::persistence::connection::SqliteDb;
use iot_bee::infrastructure::persistence::repositories::pipeline_repository::PipelineStoreRepository;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use iot_bee::application::validation_schemas_cases::cases::{
    SchemaValidationUseCases, SchemaValidationUseCasesImpl,
};

use iot_bee::adapters::api::connection_types::routers as connection_routers;
use iot_bee::adapters::api::error::ErrorResponse;
use iot_bee::adapters::api::validation_schemas::models::{
    CreateValidationSchemaRequest, UpdateValidationSchemaRequestJson,
    UpdateValidationSchemaRequestName, ValidationSchemaByIdResponse, ValidationSchemaResponse,
};
use iot_bee::adapters::api::validation_schemas::routers as validation_routers;

//para los casos de uso de connection types
use iot_bee::application::connection_types_cases::cases::ConnectionTypesUseCases;
use iot_bee::application::connection_types_cases::cases::ConnectionTypesUseCasesImpl;

//para los caso de uso de  data sources
use iot_bee::adapters::api::data_sources::routers as data_sources_routers;
use iot_bee::adapters::api::data_sources::routers::data_sources_scope;
use iot_bee::application::data_sources_cases::cases::DataSourcesUseCases;
use iot_bee::application::data_sources_cases::cases::DataSourcesUseCasesImpl;

//para los casos de uso de pipeline groups
use iot_bee::adapters::api::pipeline_groups::routers as pipeline_groups_routers;
use iot_bee::adapters::api::pipeline_groups::routers::pipeline_groups_scope;
use iot_bee::application::groups_cases::cases::PipelineGroupUseCases;
use iot_bee::application::groups_cases::cases::PipelineGroupUseCasesImpl;

// para los casos de uso de data stores
use iot_bee::adapters::api::data_store::routers as data_store_routers;
use iot_bee::adapters::api::data_store::routers::data_store_scope;
use iot_bee::application::data_store_cases::cases::DataStoreUseCases;
use iot_bee::application::data_store_cases::cases::DataStoreUseCasesImpl;


//para los casos de pipeline data 
use iot_bee::adapters::api::pipeline_data::routers as pipeline_data_routers;
use iot_bee::adapters::api::pipeline_data::routers::pipeline_data_scope;
use iot_bee::application::pipeline_data_cases::cases::PipelineDataUseCases;
use iot_bee::application::pipeline_data_cases::cases::PipelineDataUseCasesImpl;

#[derive(OpenApi)]
#[openapi(
    paths(
        //routes for validation schemas
        validation_routers::create_validation_schema,
        validation_routers::get_validation_schema,
        validation_routers::list_validation_schemas,
        validation_routers::update_validation_schema,
        validation_routers::update_validation_schema_json,
        validation_routers::delete_validation_schema,
        // routes for connection types
        connection_routers::get_connection_types,
        //routes for data sources
        data_sources_routers::create_data_source,
        data_sources_routers::get_data_source,
        data_sources_routers::list_data_sources,
        data_sources_routers::update_data_source_name,
        data_sources_routers::update_data_source,
        //routes for pipeline groups
        pipeline_groups_routers::create_pipeline_group,
        pipeline_groups_routers::get_pipeline_groups,
        pipeline_groups_routers::get_pipeline_group_by_id,
        //routes for data stores
        data_store_routers::create_data_store,
        data_store_routers::get_data_store,
        data_store_routers::list_data_stores,
        // routes for pipeline data
        pipeline_data_routers::create_pipeline_data,
        pipeline_data_routers::get_pipeline_data,
        pipeline_data_routers::get_pipeline_data_by_id,
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
        (name = "Validation Schemas", description = "CRUD operations for pipeline validation schemas"),
        (name = "Connection Types", description = "Endpoint to get all connection types"),
        (name = "Data Sources", description = "CRUD operations for data sources"),
        (name = "Pipeline Groups", description = "CRUD operations for pipeline groups"),
        (name = "Data Stores", description = "CRUD operations for data stores"),
        (name = "Pipelines", description = "CRUD operations for pipelines")
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

    //casos de uso para el data sources
    let data_sources_use_case: Arc<dyn DataSourcesUseCases + Send + Sync> =
        Arc::new(DataSourcesUseCasesImpl::new(repo.clone()));
    let data_sources_use_case_data = web::Data::from(data_sources_use_case.clone());

    //casos de uso para pipeline groups
    let pipeline_groups_use_case: Arc<dyn PipelineGroupUseCases + Send + Sync> =
        Arc::new(PipelineGroupUseCasesImpl::new(repo.clone()));
    let pipeline_groups_use_case_data = web::Data::from(pipeline_groups_use_case.clone());

    //casos de uso para data stores
    let data_store_use_case: Arc<dyn DataStoreUseCases + Send + Sync> =
        Arc::new(DataStoreUseCasesImpl::new(repo.clone()));
    let data_store_use_case_data = web::Data::from(data_store_use_case.clone());

    // casos de uso para pipeline data
    let pipeline_data_use_case: Arc<dyn PipelineDataUseCases + Send + Sync> =
        Arc::new(PipelineDataUseCasesImpl::new(repo.clone()));
    let pipeline_data_use_case_data = web::Data::from(pipeline_data_use_case.clone());


    println!("IoT Bee is starting on http://127.0.0.1:8080");
    println!("Swagger UI available at http://127.0.0.1:8080/swagger-ui/");
    HttpServer::new(move || {
        App::new()
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .service(validation_schemas_scope(
                validation_schemas_use_case_data.clone(),
            ))
            .service(connection_types_scope(
                connection_types_use_case_data.clone(),
            ))
            .service(data_sources_scope(data_sources_use_case_data.clone()))
            .service(pipeline_groups_scope(pipeline_groups_use_case_data.clone()))
            .service(data_store_scope(data_store_use_case_data.clone()))
            .service(pipeline_data_scope(pipeline_data_use_case_data.clone()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
