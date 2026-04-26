// use crate::infrastructure::persistence::repositories::pipeline_repository::PipelineStoreRepository;
// use crate::application::validation_schemas_cases::cases::{
//     SchemaValidationUseCases, SchemaValidationUseCasesImpl,
// };

// //para los casos de uso de connection types
use crate::application::connection_types_cases::cases::ConnectionTypesUseCases;
use crate::application::connection_types_cases::cases::ConnectionTypesUseCasesImpl;
use crate::infrastructure::persistence::repositories::connection_types_repository::ConnectionTypesRepository;

// para los casos de uso de validation schemas
use crate::application::validation_schemas_cases::cases::{
    SchemaValidationUseCases, SchemaValidationUseCasesImpl,
};
use crate::infrastructure::persistence::repositories::validation_schemas_repository::ValidationSchemaRepository;

//para los caso de uso de  data sources
use crate::application::data_sources_cases::cases::DataSourcesUseCases;
use crate::application::data_sources_cases::cases::DataSourcesUseCasesImpl;
use crate::infrastructure::persistence::repositories::data_source_repository::DataSourceRepository;

//para los casos de uso de pipeline groups
use crate::application::groups_cases::cases::PipelineGroupUseCases;
use crate::application::groups_cases::cases::PipelineGroupUseCasesImpl;
use crate::infrastructure::persistence::repositories::groups_repository::GroupRepository;
// // para los casos de uso de data stores
use crate::application::data_store_cases::cases::DataStoreUseCases;
use crate::application::data_store_cases::cases::DataStoreUseCasesImpl;
use crate::infrastructure::persistence::repositories::data_store_repository::DataStoreRepository;

// //para los casos de pipeline data
use crate::application::pipeline_data_cases::cases::PipelineDataUseCases;
use crate::application::pipeline_data_cases::cases::PipelineDataUseCasesImpl;
use crate::infrastructure::persistence::repositories::pipeline_data_repository::PipelineDataRepository;

use actix_web::web;

use crate::infrastructure::persistence::connection::InternalDataBase;
use std::sync::Arc;

use crate::config::Config;

pub struct AppState {
    internal_data_base: Arc<InternalDataBase>,
}

impl AppState {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = Config::get();
        let internal_data_base = Arc::new(InternalDataBase::new(&config.database_url).await?);
        Ok(Self { internal_data_base })
    }

    pub fn connection_types_app_state(
        &self,
    ) -> web::Data<dyn ConnectionTypesUseCases + Send + Sync> {
        let type_connection_repo: Arc<ConnectionTypesRepository> = Arc::new(
            ConnectionTypesRepository::new(self.internal_data_base.clone()),
        );
        let connection_types_use_case: Arc<dyn ConnectionTypesUseCases + Send + Sync> =
            Arc::new(ConnectionTypesUseCasesImpl::new(type_connection_repo));
        web::Data::from(connection_types_use_case)
    }
    pub fn validation_schemas_app_state(
        &self,
    ) -> web::Data<dyn SchemaValidationUseCases + Send + Sync> {
        let validation_schema_repo: Arc<ValidationSchemaRepository> = Arc::new(
            ValidationSchemaRepository::new(self.internal_data_base.clone()),
        );
        let validation_schema_use_case: Arc<dyn SchemaValidationUseCases + Send + Sync> =
            Arc::new(SchemaValidationUseCasesImpl::new(validation_schema_repo));
        web::Data::from(validation_schema_use_case)
    }
    pub fn data_sources_app_state(&self) -> web::Data<dyn DataSourcesUseCases + Send + Sync> {
        let data_sources_repo: Arc<DataSourceRepository> =
            Arc::new(DataSourceRepository::new(self.internal_data_base.clone()));
        let data_sources_use_case: Arc<dyn DataSourcesUseCases + Send + Sync> =
            Arc::new(DataSourcesUseCasesImpl::new(data_sources_repo));
        web::Data::from(data_sources_use_case)
    }
    pub fn pipeline_groups_app_state(&self) -> web::Data<dyn PipelineGroupUseCases + Send + Sync> {
        let pipeline_groups_repo: Arc<GroupRepository> =
            Arc::new(GroupRepository::new(self.internal_data_base.clone()));
        let pipeline_groups_use_case: Arc<dyn PipelineGroupUseCases + Send + Sync> =
            Arc::new(PipelineGroupUseCasesImpl::new(pipeline_groups_repo));
        web::Data::from(pipeline_groups_use_case)
    }

    pub fn data_stores_app_state(&self) -> web::Data<dyn DataStoreUseCases + Send + Sync> {
        let data_stores_repo: Arc<DataStoreRepository> =
            Arc::new(DataStoreRepository::new(self.internal_data_base.clone()));
        let data_stores_use_case: Arc<dyn DataStoreUseCases + Send + Sync> =
            Arc::new(DataStoreUseCasesImpl::new(data_stores_repo));
        web::Data::from(data_stores_use_case)
    }

    pub fn pipeline_data_app_state(&self) -> web::Data<dyn PipelineDataUseCases + Send + Sync> {
        let pipeline_data_repo: Arc<PipelineDataRepository> =
            Arc::new(PipelineDataRepository::new(self.internal_data_base.clone()));
        let pipeline_data_use_case: Arc<dyn PipelineDataUseCases + Send + Sync> =
            Arc::new(PipelineDataUseCasesImpl::new(pipeline_data_repo));
        web::Data::from(pipeline_data_use_case)
    }
}
