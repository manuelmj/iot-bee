use actix::fut::wrap_future;
use actix::prelude::*;
use tokio::sync::mpsc;

use crate::adapters::actor_system::pipeline_actor_module::consumer_actor::{
    data_consumer_actor::DataConsumerActor,
    messages::{ConsumerActorAction, ConsumerActorActionMessage, ConsumerActorState},
};
use crate::adapters::actor_system::pipeline_actor_module::general_ports::SendDataToProcessor;
use crate::domain::entities::data_consumer_types::DataConsumerRawType;
use crate::domain::outbound::data_source::DataSource;
use crate::logging::AppLogger;

static LOGGER: AppLogger = AppLogger::new(
    "iot_bee::adapters::actor_system::pipeline_actor_module::consumer_actor::handlers",
);

const CHANNEL_CAPACITY: usize = 100;

impl<T, U> Handler<ConsumerActorActionMessage> for DataConsumerActor<T, U>
where
    T: DataSource + Send + Sync + 'static,
    U: SendDataToProcessor + Send + Sync + 'static,
{
    type Result = ResponseActFuture<Self, ConsumerActorState>;

    fn handle(&mut self, msg: ConsumerActorActionMessage, ctx: &mut Context<Self>) -> Self::Result {
        match msg.action() {
            ConsumerActorAction::StartConsuming => {
                if self.state() == ConsumerActorState::Consuming {
                    LOGGER
                        .warn("StartConsuming received but actor is already consuming. Ignoring.");
                    return Box::pin(async { ConsumerActorState::Consuming }.into_actor(self));
                }

                let (tx, rx) = mpsc::channel::<DataConsumerRawType>(CHANNEL_CAPACITY);
                self.set_sender(Some(tx.clone()));

                let data_source = self.data_source();
                let actor_addr = ctx.address();
                let sender_to_processor = self.data_processor();
                tokio::spawn(async move {
                    let mut rx = rx;
                    LOGGER.info("DataConsumerActor started consuming data from DataSource...");
                    while let Some(data) = rx.recv().await {
                        // LOGGER.info(&format!("Received data from DataSource: {:?}", data));
                        if let Err(e) = sender_to_processor.send(&data).await {
                            LOGGER.error(&format!("Failed to send data to processor: {}", e));
                        }
                    }
                    LOGGER.info("DataConsumerActor channel closed, stopping consumption.");
                    actor_addr.do_send(ConsumerActorActionMessage::channel_died());
                });

                Box::pin(
                    wrap_future::<_, Self>(async move { data_source.start_to_consume(tx).await })
                        .map(|result, actor, _ctx| {
                            match result {
                                Ok(_) => {
                                    actor.set_state(ConsumerActorState::Consuming);
                                    LOGGER.info("Consumer started successfully");
                                }
                                Err(e) => {
                                    actor.set_state(ConsumerActorState::Idle);
                                    actor.set_sender(None);
                                    LOGGER.error(&format!("Failed to start consuming: {}", e));
                                }
                            }
                            actor.state()
                        }),
                )
            }

            ConsumerActorAction::StopConsuming => {
                if self.state() == ConsumerActorState::Stopped
                    || self.state() == ConsumerActorState::Stopping
                {
                    LOGGER.warn(
                        "StopConsuming received but actor is already stopping/stopped. Ignoring.",
                    );
                    let state = self.state();
                    return Box::pin(async move { state }.into_actor(self));
                }

                // Soltar el sender dispara sender.closed() en el task de RabbitMQ.
                self.set_sender(None);
                self.set_state(ConsumerActorState::Stopping);
                LOGGER
                    .info("StopConsuming received. Sender dropped, RabbitMQ task will shut down.");

                Box::pin(async { ConsumerActorState::Stopping }.into_actor(self))
            }

            ConsumerActorAction::ChannelDied => {
                match self.state() {
                    ConsumerActorState::Consuming => {
                        LOGGER.warn(
                            "ChannelDied received while consuming. Transitioning to Reconnecting.",
                        );
                        self.set_state(ConsumerActorState::Reconnecting);
                        self.set_sender(None);
                        ctx.address()
                            .do_send(ConsumerActorActionMessage::start_consuming());
                    }
                    ConsumerActorState::Stopped | ConsumerActorState::Stopping => {
                        LOGGER.warn("Channel closed after stopping/stopped action. Ignoring.");
                    }
                    _ => {
                        LOGGER.warn(
                            "ChannelDied received but actor is not in Consuming state. Ignoring.",
                        );
                    }
                }
                let state = self.state();
                Box::pin(async move { state }.into_actor(self))
            }

            ConsumerActorAction::GetState => {
                let state = self.state();
                Box::pin(async move { state }.into_actor(self))
            }
        }
    }
}

use crate::adapters::actor_system::pipeline_actor_module::general_messages::{
    ActorActions, ResponseActorActionMessage, SendActorActionMessage, SendActorActionMessageResult,
};

impl<T, U> Handler<SendActorActionMessage> for DataConsumerActor<T, U>
where
    T: DataSource + Send + Sync + 'static,
    U: SendDataToProcessor + Send + Sync + 'static,
{
    type Result = ResponseFuture<SendActorActionMessageResult>;

    fn handle(&mut self, msg: SendActorActionMessage, ctx: &mut Self::Context) -> Self::Result {
        match msg.action() {
            ActorActions::Stop => {
                LOGGER.info("DataConsumerActor: Stop action received.");
                ctx.address()
                    .do_send(ConsumerActorActionMessage::stop_consuming());
                Box::pin(async { Ok(ResponseActorActionMessage::stopped()) })
            }
            ActorActions::Restart => {
                LOGGER.info("DataConsumerActor: Restart action received.");
                ctx.address()
                    .do_send(ConsumerActorActionMessage::stop_consuming());
                ctx.address()
                    .do_send(ConsumerActorActionMessage::start_consuming());
                Box::pin(async { Ok(ResponseActorActionMessage::restarting()) })
            }
            ActorActions::Status => {
                LOGGER.info("DataConsumerActor: Status action received.");
                let current = self.state();
                Box::pin(async move {
                    let status = match current {
                        ConsumerActorState::Consuming | ConsumerActorState::Reconnecting => {
                            ResponseActorActionMessage::running()
                        }
                        ConsumerActorState::Stopped | ConsumerActorState::Stopping => {
                            ResponseActorActionMessage::stopped()
                        }
                        ConsumerActorState::Idle => ResponseActorActionMessage::running(),
                    };
                    Ok(status)
                })
            }
            _ => {
                LOGGER.warn("DataConsumerActor: Unknown action received.");
                Box::pin(async { Ok(ResponseActorActionMessage::failed()) })
            }
        }
    }
}
