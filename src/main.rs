use actix_web::{web, App, HttpServer};

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

use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;
use iot_bee::adapters::api::api_docs::ApiDoc;
use iot_bee::logging::{AppLogger, init_tracing};

static LOGGER: AppLogger = AppLogger::new("iot_bee::main");


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    init_tracing();
    banner();

    let app_store = match AppState::new().await {
        Ok(state) => state,
        Err(err) => {
            LOGGER.error(&format!("Failed to initialize application state: {err}"));
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

    LOGGER.info("IoT Bee is starting on http://127.0.0.1:8080");
    LOGGER.info("Swagger UI available at http://127.0.0.1:8080/swagger-ui/");
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



use tracing::info; 
fn banner() {
    info!("========================================");
    info!("  ____      _______    ____             ");
    info!(" |_  _| ___|__   __|  | __ )  ___  ___  ");
    info!("   |||/ _ \\  | |______|  _ \\ / _ \\/ _ \\ ");
    info!("   ||| (_) | | |______| |_) |  __/  __/ ");
    info!("  _|_|\\___/  |_|      |____/ \\___|\\___| ");
    info!("               IoT Bee                    ");
    info!("========================================");
}