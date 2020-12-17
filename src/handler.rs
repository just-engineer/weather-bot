use teloxide::prelude::*;
use crate::database::Datastore;
use crate::client_message::ClientMessage;
use std::sync::Arc;
use crate::weather_settings::{WeatherSettings, Location};
use mongodb::bson::doc;
use mongodb::bson::Document;


pub async fn message_handler(cx: UpdateWithCx<Message>, database: Arc<Datastore>) -> Result<(), RequestError> {
    let chat_id = cx.update.chat_id().to_string();
    let weather_settings = WeatherSettings::new(chat_id);
    if let Some(text) = cx.update.text() {

        database.upsert(where_chat_id(&weather_settings), weather_settings).await;

    } else if let Some(location) = cx.update.location() {
        let latitude = location.latitude;
        let longitude = location.longitude;

        let message = format!("Received location, lat:{}, long:{}", &latitude, &longitude);

        let location_settings = weather_settings
            .with_location(Location::new(latitude, longitude));

        database.upsert(where_chat_id(&location_settings), location_settings).await;

        cx.answer_str(message).await.unwrap();
    } else {
        cx.answer_str("Send me a text or a location").await?;
    }
    Ok(())
}

fn where_chat_id(weather_settings: &WeatherSettings) -> Document {
    let chat_id = weather_settings.chat_id.as_str();
    doc! {"chat_id":  chat_id}
}
