use actix_web::{web, App, HttpServer};
use iot_bee::adapters::api::connection_types::routers as connection_routers;
use iot_bee::adapters::api::data_sources::{routers as data_sources_routers};
use iot_bee::adapters::api::data_store::routers as data_store_routers;
use iot_bee::adapters::api::error::ErrorResponse;
use iot_bee::adapters::api::pipeline_data::routers as pipeline_data_routers;
use iot_bee::adapters::api::pipeline_groups::{routers as pipeline_groups_routers};
use iot_bee::adapters::api::validation_schemas::models::{
    CreateValidationSchemaRequest, UpdateValidationSchemaRequestJson,
    UpdateValidationSchemaRequestName, ValidationSchemaByIdResponse, ValidationSchemaResponse,
};
use iot_bee::adapters::api::validation_schemas::routers as validation_routers;
//USE CASES
//connection types
use iot_bee::adapters::api::connection_types::routers::connection_types_scope;
use iot_bee::application::connection_types_cases::cases::ConnectionTypesUseCases;
//validation schemas
use iot_bee::adapters::api::validation_schemas::routers::validation_schemas_scope;
use iot_bee::application::validation_schemas_cases::cases::SchemaValidationUseCases;
//data sources
use iot_bee::adapters::api::data_sources::routers::data_sources_scope;
use iot_bee::application::data_sources_cases::cases::DataSourcesUseCases;
// pipeline groups
use iot_bee::adapters::api::pipeline_groups::routers::pipeline_groups_scope;
use iot_bee::application::groups_cases::cases::PipelineGroupUseCases;
//data stores
use iot_bee::adapters::api::data_store::routers::data_store_scope;
use iot_bee::application::data_store_cases::cases::DataStoreUseCases;
//pipeline data
use iot_bee::adapters::api::pipeline_data::routers::pipeline_data_scope;
use iot_bee::application::pipeline_data_cases::cases::PipelineDataUseCases;


use iot_bee::composition::api_composition::api_composer::AppState;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        validation_routers::create_validation_schema,
        validation_routers::get_validation_schema,
        validation_routers::list_validation_schemas,
        validation_routers::update_validation_schema,
        validation_routers::update_validation_schema_json,
        validation_routers::delete_validation_schema,
        connection_routers::get_connection_types,
        data_sources_routers::create_data_source,
        data_sources_routers::get_data_source,
        data_sources_routers::list_data_sources,
        data_sources_routers::update_data_source_name,
        data_sources_routers::update_data_source,
        pipeline_groups_routers::create_pipeline_group,
        pipeline_groups_routers::get_pipeline_groups,
        pipeline_groups_routers::get_pipeline_group_by_id,
        data_store_routers::create_data_store,
        data_store_routers::get_data_store,
        data_store_routers::list_data_stores,
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

    let app_store = match AppState::new().await {
        Ok(state) => state,
        Err(err) => {
            eprintln!("Failed to initialize application state: {}", err);
            std::process::exit(1);
        }
    };

    let connection_types: web::Data<dyn ConnectionTypesUseCases + Send + Sync> =
        app_store.connection_types_app_state();

    let validation_schemas: web::Data<dyn SchemaValidationUseCases + Send + Sync> =
        app_store.validation_schemas_app_state();

    let data_sources: web::Data<dyn DataSourcesUseCases + Send + Sync> =
        app_store.data_sources_app_state();
    
    let pipeline_groups: web::Data<dyn PipelineGroupUseCases + Send + Sync> =
        app_store.pipeline_groups_app_state();

    let data_stores: web::Data<dyn DataStoreUseCases + Send + Sync> =
        app_store.data_stores_app_state();

    let pipeline_data: web::Data<dyn PipelineDataUseCases + Send + Sync> =
        app_store.pipeline_data_app_state();

    println!("IoT Bee is starting on http://127.0.0.1:8080");
    println!("Swagger UI available at http://127.0.0.1:8080/swagger-ui/");
    HttpServer::new(move || {
        App::new()
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .service(connection_types_scope(connection_types.clone()))
            .service(validation_schemas_scope(validation_schemas.clone()))
            .service(data_sources_scope(data_sources.clone()))
            .service(pipeline_groups_scope(pipeline_groups.clone()))
            .service(data_store_scope(data_stores.clone()))
            .service(pipeline_data_scope(pipeline_data.clone()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
