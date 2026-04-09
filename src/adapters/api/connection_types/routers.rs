use crate::application::connection_types_cases::cases::ConnectionTypesUseCases;
use crate::domain::error::IoTBeeError;
use actix_web::{HttpResponse, get,web};

type UseCase = dyn ConnectionTypesUseCases + Send + Sync;

pub fn connection_types_scope(use_cases: web::Data<UseCase>) -> actix_web::Scope {
    web::scope("/connection-types")
        .app_data(use_cases)
        .service(get_connection_types)
}

use crate::adapters::api::connection_types::models::ConnectionTypeResponse;

#[utoipa::path(
    get,
    path = "/connection-types",
    responses(
        (status = 200, description = "List of connection types", body = [ConnectionTypeResponse])
    ),
    tag = "Connection Types"
)]
#[get("")]
pub async fn get_connection_types(
    use_cases: web::Data<UseCase>,
) -> Result<HttpResponse, IoTBeeError> {
    let connection_types = use_cases.get_all_connection_types().await?;

    let connection_types = connection_types
        .into_iter()
        .map(|data| data.into())
        .collect::<Vec<ConnectionTypeResponse>>();

    Ok(HttpResponse::Ok().json(connection_types))
}
