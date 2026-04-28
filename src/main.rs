use iot_bee::composition::api_composition::api_composer::ApiComposer;
use logging::init_tracing;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_tracing();
    ApiComposer::run().await
}

