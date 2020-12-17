use std::sync::Arc;

use teloxide::{prelude::*, utils::command::BotCommand};

use crate::database::Datastore;
use crate::scheduler::Scheduler;

mod database;
mod client_message;
mod handler;
mod weather_settings;
mod scheduler;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a location name.")]
    Location(String),
    #[command(description = "handle a time.", parse_with = "split")]
    Time { hour: u8, minute: u8 },
    #[command(description = "Print weather")]
    Print,
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();

    let datastore = Arc::new(Datastore::init("mongodb://localhost:27017").await);

    log::info!("Starting simple_commands_bot...");

    let bot = Bot::from_env();
     let handler = Arc::new(handler::message_handler);

    let mut scheduler = Scheduler::new(datastore.clone());
    let dispatcher = Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<Message>| {
            rx.for_each_concurrent(None, move |message| {
                let handler = handler.clone();
                let database = datastore.clone();
                async move {
                    handler(message, database).await.log_on_error().await;
                }
            })
        });

    tokio::join!(dispatcher.dispatch(), scheduler.run());
}


