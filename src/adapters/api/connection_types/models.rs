use crate::application::connection_types_cases::cases::ConnectionType;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ConnectionTypeResponse {
    #[serde(rename = "id")]
    pub id: u32,
    #[serde(rename = "connectionType")]
    pub connection_type: String,
}

impl From<ConnectionType> for ConnectionTypeResponse {
    fn from(use_case: ConnectionType) -> Self {
        ConnectionTypeResponse {
            id: use_case.id,
            connection_type: use_case.connection_type,
        }
    }
}
