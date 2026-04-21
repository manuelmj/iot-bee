/// Test de integración del PipelineSupervisor.
///
/// Verifica que el supervisor gestiona correctamente el ciclo de vida de los
/// pipelines registrados: add, remove, list, stop, restart y status.
///
/// ── Arquitectura de los dobles ───────────────────────────────────────────────
///
///   FakeDataSource → DataConsumerActor → DataProcessorActor → DataStoreActor
///                         │ ConsumerActorBridge   │ ProcessorActorBridge   │ StoreActorBridge
///                         ▼                       ▼                        ▼
///                    (T = consumer)          (U = processor)          (V = store)
///                     en PipelineAbstractionController
///
/// Los tres actores son independientes entre sí en este test (no están
/// conectados por el flujo de datos), porque aquí solo se prueba la capa del
/// supervisor: add, remove, list, stop, restart y status.
/// El flujo de datos (consumer → processor → store) está cubierto por
/// el test `pipeline_integration`.
use actix::prelude::*;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

use iot_bee::adapters::actor_system::pipeline_actor_module::{
    consumer_actor::data_consumer_actor::{ConsumerActorBridge, DataConsumerActor},
    general_messages::ActorStatus,
    processor_actor::data_processor_actor::{DataProcessorActor, ProcessorActorBridge},
    store_actor::data_store_actor::{DataStoreActor, StoreActorBridge},
};
use iot_bee::adapters::actor_system::supervisor_pipeline_life_time::{
    messges::{
        AddPipelineMessage, ListPipelinesMessage, RemovePipelineMessage, RestartPipelineMessage,
        StatusPipelineMessage, StopPipelineMessage,
    },
    pipeline_abstraction::PipelineAbstractionController,
    pipeline_supervisor::PipelineSupervisor,
};
// Alias de tipos concretos solo para los helpers, no para TestController/TestSupervisor.
type StoreBridge = StoreActorBridge<FakeExternalStore>;
type ProcessorBridge = ProcessorActorBridge<StoreBridge>;
type ConsumerBridge = ConsumerActorBridge<FakeDataSource, ProcessorBridge>;
// El supervisor ya no necesita parámetros de tipo.
type TestSupervisor = PipelineSupervisor;
use iot_bee::domain::entities::data_consumer_types::DataConsumerRawType;
use iot_bee::domain::error::IoTBeeError;
use iot_bee::domain::outbound::{
    data_external_store::DataExternalStore,
    data_source::DataSource,
};
use iot_bee::logging::init_tracing;
use iot_bee::logging::AppLogger;

// ── Dobles de prueba ──────────────────────────────────────────────────────────

struct FakeExternalStore;

#[async_trait]
impl DataExternalStore for FakeExternalStore {
    async fn save(&self, _data: DataConsumerRawType) -> Result<(), IoTBeeError> {
        Ok(())
    }
}

/// Fuente de datos que retorna inmediatamente sin emitir ningún dato.
/// El consumer queda en estado Consuming (el sender interno lo mantiene vivo)
/// sin generar el bucle de reconexión, lo que es suficiente para testar
/// el ciclo de vida en el supervisor.
struct FakeDataSource;

#[async_trait]
impl DataSource for FakeDataSource {
    async fn start_to_consume(
        &self,
        _sender: Sender<DataConsumerRawType>,
    ) -> Result<(), IoTBeeError> {
        Ok(())
    }
}

// ── Alias de tipos ──────────────────────────────────────────────────────────── 

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Inicia un DataStoreActor y devuelve su bridge.
fn crear_store_bridge() -> StoreBridge {
    let addr = DataStoreActor::new(Arc::new(FakeExternalStore)).start();
    StoreActorBridge::new(addr)
}

/// Inicia un DataProcessorActor (con su propio store interno) y devuelve su bridge.
fn crear_processor_bridge() -> ProcessorBridge {
    let store_addr = DataStoreActor::new(Arc::new(FakeExternalStore)).start();
    let store_bridge = Arc::new(StoreActorBridge::new(store_addr));
    let processor_addr = DataProcessorActor::new(store_bridge).start();
    ProcessorActorBridge::new(processor_addr)
}

/// Inicia un DataConsumerActor (con FakeDataSource y su propio processor interno)
/// y devuelve su bridge.
fn crear_consumer_bridge() -> ConsumerBridge {
    let store_addr = DataStoreActor::new(Arc::new(FakeExternalStore)).start();
    let store_bridge = Arc::new(StoreActorBridge::new(store_addr));
    let processor_addr = DataProcessorActor::new(store_bridge).start();
    let processor_bridge = Arc::new(ProcessorActorBridge::new(processor_addr));
    let consumer_addr =
        DataConsumerActor::new(Arc::new(FakeDataSource), processor_bridge).start();
    ConsumerActorBridge::new(consumer_addr)
}

/// Construye un controller con consumer, processor y store independientes.
fn crear_controller() -> PipelineAbstractionController {
    PipelineAbstractionController::new(
        Box::new(crear_consumer_bridge()),
        Box::new(crear_processor_bridge()),
        Box::new(crear_store_bridge()),
    )
}

// ── Tests: gestión del registro ───────────────────────────────────────────────

#[actix_rt::test]
async fn supervisor_add_pipeline_registra_correctamente() {
    init_tracing();
    let supervisor = TestSupervisor::new().start();

    let resultado = supervisor
        .send(AddPipelineMessage::new(1, crear_controller()))
        .await
        .unwrap();

    assert!(resultado.is_ok(), "Add debe retornar Ok(())");
}

#[actix_rt::test]
async fn supervisor_add_pipeline_id_duplicado_retorna_error() {
    init_tracing();
    let supervisor = TestSupervisor::new().start();

    supervisor
        .send(AddPipelineMessage::new(1, crear_controller()))
        .await
        .unwrap()
        .unwrap();

    let resultado = supervisor
        .send(AddPipelineMessage::new(1, crear_controller()))
        .await
        .unwrap();

    assert!(
        resultado.is_err(),
        "Registrar el mismo id dos veces debe retornar Err"
    );
}

#[actix_rt::test]
async fn supervisor_list_retorna_ids_registrados() {
    init_tracing();
    static LOGGER: AppLogger = AppLogger::new(
        "test::supervisor_actor::supervisor_list_retorna_ids_registrados",
    );
    let supervisor = TestSupervisor::new().start();

    supervisor
        .send(AddPipelineMessage::new(10, crear_controller()))
        .await
        .unwrap()
        .unwrap();
    supervisor
        .send(AddPipelineMessage::new(20, crear_controller()))
        .await
        .unwrap()
        .unwrap();
    supervisor
        .send(AddPipelineMessage::new(30, crear_controller()))
        .await
        .unwrap()
        .unwrap();

    let mut ids = supervisor
        .send(ListPipelinesMessage)
        .await
        .unwrap()
        .unwrap();
    ids.sort(); // HashMap no garantiza orden

    LOGGER.info(&format!("IDs registrados: {:?}", ids));

    assert_eq!(ids, vec![10, 20, 30]);
}

#[actix_rt::test]
async fn supervisor_list_sin_pipelines_retorna_vec_vacio() {
    init_tracing();
    let supervisor = TestSupervisor::new().start();

    let ids = supervisor
        .send(ListPipelinesMessage)
        .await
        .unwrap()
        .unwrap();

    assert!(ids.is_empty(), "Sin pipelines registrados la lista debe estar vacía");
}

#[actix_rt::test]
async fn supervisor_remove_pipeline_elimina_del_registro() {
    init_tracing();
    let supervisor = TestSupervisor::new().start();

    supervisor
        .send(AddPipelineMessage::new(1, crear_controller()))
        .await
        .unwrap()
        .unwrap();

    let resultado = supervisor
        .send(RemovePipelineMessage::new(1))
        .await
        .unwrap();
    assert!(resultado.is_ok(), "Remove debe retornar Ok(())");

    let ids = supervisor
        .send(ListPipelinesMessage)
        .await
        .unwrap()
        .unwrap();
    assert!(
        ids.is_empty(),
        "La lista debe quedar vacía tras eliminar el único pipeline"
    );
}

#[actix_rt::test]
async fn supervisor_remove_pipeline_inexistente_retorna_error() {
    init_tracing();
    let supervisor = TestSupervisor::new().start();

    let resultado = supervisor
        .send(RemovePipelineMessage::new(99))
        .await
        .unwrap();

    assert!(resultado.is_err(), "Eliminar un id inexistente debe retornar Err");
}

// ── Tests: acciones de ciclo de vida ─────────────────────────────────────────

#[actix_rt::test]
async fn supervisor_stop_pipeline_delega_stop_a_los_tres_actores() {
    init_tracing();
    let supervisor = TestSupervisor::new().start();
    supervisor
        .send(AddPipelineMessage::new(1, crear_controller()))
        .await
        .unwrap()
        .unwrap();

    let (r_consumer, r_processor, r_store) = supervisor
        .send(StopPipelineMessage::new(1))
        .await
        .unwrap()  // MailboxError
        .unwrap(); // Result<TripleResult, IoTBeeError>

    assert_eq!(
        r_consumer.unwrap().status(),
        ActorStatus::Stopped,
        "El actor consumer debe reportar Stopped"
    );
    assert_eq!(
        r_processor.unwrap().status(),
        ActorStatus::Stopped,
        "El actor processor debe reportar Stopped"
    );
    assert_eq!(
        r_store.unwrap().status(),
        ActorStatus::Stopped,
        "El actor store debe reportar Stopped"
    );
}

#[actix_rt::test]
async fn supervisor_stop_pipeline_inexistente_retorna_error() {
    init_tracing();
    let supervisor = TestSupervisor::new().start();

    let resultado = supervisor
        .send(StopPipelineMessage::new(99))
        .await
        .unwrap();

    assert!(
        resultado.is_err(),
        "Stop sobre un id inexistente debe retornar Err"
    );
}

#[actix_rt::test]
async fn supervisor_restart_pipeline_delega_restart_a_los_tres_actores() {
    init_tracing();
    let supervisor = TestSupervisor::new().start();
    supervisor
        .send(AddPipelineMessage::new(1, crear_controller()))
        .await
        .unwrap()
        .unwrap();

    let (r_consumer, r_processor, r_store) = supervisor
        .send(RestartPipelineMessage::new(1))
        .await
        .unwrap()
        .unwrap();

    assert_eq!(
        r_consumer.unwrap().status(),
        ActorStatus::Restarting,
        "El actor consumer debe reportar Restarting"
    );
    assert_eq!(
        r_processor.unwrap().status(),
        ActorStatus::Restarting,
        "El actor processor debe reportar Restarting"
    );
    assert_eq!(
        r_store.unwrap().status(),
        ActorStatus::Restarting,
        "El actor store debe reportar Restarting"
    );
}

#[actix_rt::test]
async fn supervisor_status_pipeline_retorna_running_para_actores_activos() {
    init_tracing();
    let supervisor = TestSupervisor::new().start();
    supervisor
        .send(AddPipelineMessage::new(1, crear_controller()))
        .await
        .unwrap()
        .unwrap();

    let (r_consumer, r_processor, r_store) = supervisor
        .send(StatusPipelineMessage::new(1))
        .await
        .unwrap()
        .unwrap();

    assert_eq!(
        r_consumer.unwrap().status(),
        ActorStatus::Running,
        "El actor consumer debe reportar Running"
    );
    assert_eq!(
        r_processor.unwrap().status(),
        ActorStatus::Running,
        "El actor processor debe reportar Running"
    );
    assert_eq!(
        r_store.unwrap().status(),
        ActorStatus::Running,
        "El actor store debe reportar Running"
    );
}
