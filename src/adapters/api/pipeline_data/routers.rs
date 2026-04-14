use crate::adapters::api::pipeline_data::models::{CreatePipelineDataRequest,PipelineDataResponse,PipelineDataId};
use crate::application::pipeline_data_cases::cases::PipelineDataUseCases;
use crate::domain::error::IoTBeeError;
use crate::domain::entities::pipeline_data::{PipelineDataInputModel, PipelineDataOutputModel};
use actix_web::{web, HttpResponse, get, post};

use crate::adapters::api::error::ErrorResponse;
type UseCase = dyn PipelineDataUseCases + Send + Sync;

pub fn pipeline_data_scope(use_case: web::Data<UseCase>) -> actix_web::Scope {
    web::scope("/pipelines")
        .app_data(use_case)
        .service(create_pipeline_data)
        .service(get_pipeline_data)
        .service(get_pipeline_data_by_id)
}

#[utoipa::path(
    post,
    path = "/pipelines",
    request_body = CreatePipelineDataRequest,
    responses(
        (status = 201, description = "Pipeline created successfully"),
        (status = 400, description = "Invalid data", body = ErrorResponse)
    ),
    tag = "Pipelines"
)]
#[post("")]
pub async fn create_pipeline_data(
    use_case: web::Data<UseCase>,
    body: web::Json<CreatePipelineDataRequest>,
) -> Result<HttpResponse, IoTBeeError> {
    let pipeline_input: CreatePipelineDataRequest = body.into_inner();
    let pipeline_input: PipelineDataInputModel = pipeline_input.try_into()?;
    use_case.create_pipeline(&pipeline_input).await?;
    Ok(HttpResponse::Created().finish())
}

#[utoipa::path(
    get,
    path = "/pipelines",
    responses(
        (status = 200, description = "Pipelines retrieved successfully", body = [PipelineDataResponse]),
        (status = 404, description = "No pipelines found", body = ErrorResponse)
    ),
    tag = "Pipelines"
)]
#[get("")]
pub async fn get_pipeline_data(
    use_case: web::Data<UseCase>,
) -> Result<HttpResponse, IoTBeeError> {
    let pipelines: Vec<PipelineDataOutputModel> = use_case.get_pipeline().await?;
    let response: Vec<PipelineDataResponse> = pipelines.into_iter().map(|p| p.try_into()).collect::<Result<_, IoTBeeError>>()?;
    Ok(HttpResponse::Ok().json(response))
}

#[utoipa::path(
    get,
    path = "/pipelines/{id}",
    params(
        ("id" = u32, Path, description = "Pipeline ID")
    ),
    responses(
        (status = 200, description = "Pipeline retrieved successfully", body = PipelineDataResponse),
        (status = 404, description = "Pipeline not found", body = ErrorResponse)
    ),
    tag = "Pipelines"
)]
#[get("/{id}")]
pub async fn get_pipeline_data_by_id(
    use_case: web::Data<UseCase>,
    id: web::Path<PipelineDataId>,
) -> Result<HttpResponse, IoTBeeError> {
    let pipeline_id: PipelineDataId = id.into_inner();
    let pipeline: PipelineDataOutputModel = use_case.get_pipeline_by_id(&pipeline_id).await?;
    let response: PipelineDataResponse = pipeline.try_into()?;
    Ok(HttpResponse::Ok().json(response))
}

