mod composition;

use actix_web::{web, App, HttpServer};
use iot_bee::infrastructure::persistence::connection::SqliteDb;
use iot_bee::infrastructure::persistence::repositories::pipeline_repository::{PipelineStoreRepository};
use iot_bee::adapters::api::validation_schemas::routers::validation_schemas_scope;
use std::sync::Arc;


use iot_bee::application::validation_schemas_cases::cases::{SchemaValidationUseCases, SchemaValidationUseCasesImpl};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    let db = SqliteDb::new(&database_url)
        .await
        .expect("Failed to connect to database");


    //implementacion concreta del repositorio de validaciones de pipeline utilizando la conexion a la base de datos
    let repo = PipelineStoreRepository::new(db);

    //uso de caso al que se inyecta el repo
    let validation_schemas_use_case: Arc<dyn SchemaValidationUseCases + Send + Sync> = Arc::new(SchemaValidationUseCasesImpl::new(repo));



    let use_case_data = web::Data::from(validation_schemas_use_case);

    println!("IoT Bee is starting on http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .service(validation_schemas_scope(use_case_data.clone()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
