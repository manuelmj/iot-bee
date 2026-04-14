use crate::adapters::api::data_store::models::{CreateDataStoreRequest, DataStoreResponse,DataStoreId};
use crate::domain::entities::data_store::{PipelineDataStoreInputModel, PipelineDataStoreOutputModel};
use crate::domain::error::IoTBeeError;
use crate::application::data_store_cases::cases::DataStoreUseCases;

use crate::adapters::api::error::ErrorResponse;
use actix_web::{web, HttpResponse,post,get};

type UseCase = dyn DataStoreUseCases + Send + Sync;


pub fn data_store_scope(use_case: web::Data<UseCase>) -> actix_web::Scope {
    web::scope("/data-stores")
        .app_data(use_case)
        .service(create_data_store)
        .service(get_data_store)
        .service(list_data_stores)
}

#[utoipa::path(
    post,
    path = "/data-stores",
    request_body = CreateDataStoreRequest,
    responses(
        (status = 201, description = "Data store created successfully"),
        (status = 400, description = "Invalid data", body = ErrorResponse)
    ),
    tag = "Data Stores"
)]
#[post("")]
pub async fn create_data_store(
    use_case: web::Data<UseCase>,
    body: web::Json<CreateDataStoreRequest>,
) -> Result<HttpResponse, IoTBeeError> {
    let data_store_input = PipelineDataStoreInputModel::try_from(body.into_inner())?;
    use_case.create_data_store(&data_store_input).await?;
    Ok(HttpResponse::Created().finish())
}

#[utoipa::path(
    get,
    path = "/data-stores/{id}",
    params(
        ("id" = i32, Path, description = "Schema ID")
    ),
    responses(
        (status = 200, description = "Data store retrieved successfully", body = DataStoreResponse),
        (status = 404, description = "Data store not found", body = ErrorResponse)
    ),
    tag = "Data Stores"
)]
#[get("/{id}")]
pub async fn get_data_store(
    use_case: web::Data<UseCase>,
    id: web::Path<DataStoreId>,
) -> Result<HttpResponse, IoTBeeError> {
    let data_id: u32 = id.into_inner();
    let data_store: PipelineDataStoreOutputModel = use_case.get_data_store_by_id(&data_id).await?;
    let response: DataStoreResponse = data_store.try_into()?;
    Ok(HttpResponse::Ok().json(response))
}

#[utoipa::path(
    get,
    path = "/data-stores",
    responses(
        (status = 200, description = "Data stores retrieved successfully", body = [DataStoreResponse])
    ),
    tag = "Data Stores"
)]
#[get("")]
pub async fn list_data_stores(
    use_case: web::Data<UseCase>,
) -> Result<HttpResponse, IoTBeeError> {
    println!("list data stores handler called");
    let data_stores: Vec<PipelineDataStoreOutputModel> = use_case.get_data_store().await?;
    let response: Vec<DataStoreResponse> = data_stores.into_iter()
    .map(|data_store|data_store.try_into())
    .collect::<Result<Vec<DataStoreResponse>, IoTBeeError>>()?; 

    Ok(HttpResponse::Ok().json(response))
}