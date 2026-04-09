use crate::adapters::api::connection_types::models::ConnectionTypeResponse;
use crate::application::connection_types_cases::cases::ConnectionType;

impl From<ConnectionType> for ConnectionTypeResponse {
    fn from(use_case: ConnectionType) -> Self {
        ConnectionTypeResponse {
            id: use_case.id,
            connection_type: use_case.connection_type,
        }
    }
}
