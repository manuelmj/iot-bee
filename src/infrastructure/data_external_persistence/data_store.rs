
use crate::domain::outbound::data_external_store::DataExternalStore;    
use asycn_trait::async_trait;

use crate::logger::LOGGER;

static LOGGER: AppLogger = AppLogger::new("iot_bee::infrastructure::data_external_persistence::data_store::InfluxDbDataExternalStore");

pub struct InfluxDbDataExternalStore; 

#[async_trait]
impl DataExternalStore for InfluxDbDataExternalStore{
    async fn save() {
        //aqui iria la logica para guardar en influxdb, pero por ahora lo dejamos asi para no complicar el ejemplo. 
        LOGGER.info("Saving data to InfluxDB... (this is a placeholder implementation)");
    }
}

