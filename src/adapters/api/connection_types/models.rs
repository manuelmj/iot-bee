use serde::Serialize;
// use validator::Validate;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ConnectionTypeResponse {
    #[serde(rename = "id")]
    pub id: u32,
    #[serde(rename = "connectionType")]
    pub connection_type: String,
}
