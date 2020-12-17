use bson::*;
use mongodb::bson;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use chrono::{Utc, DurationRound, Duration};
use crate::database::Documented;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct WeatherSettings {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub chat_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<DateTime>,
}

impl WeatherSettings {
    pub fn new(chat_id: String) -> Self {
        WeatherSettings {
            id: None,
            chat_id,
            location: None,
            time: None,
        }
    }

    pub fn with_location(self, location: Location) -> Self {
        WeatherSettings {
            location: Some(location),
            ..self
        }
    }

    pub fn with_time(self, time: DateTime) -> Self {
        WeatherSettings {
            time: Some(time),
            ..self
        }
    }

}

impl Documented for WeatherSettings {
    const COLLECTION_NAME: &'static str = "weather-settings";

    fn set_id(self, id: ObjectId) -> Self {
        WeatherSettings {
            id: Some(id),
            chat_id: self.chat_id,
            location: self.location,
            time: self.time,
        }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

impl Location {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Location {
            latitude,
            longitude,
        }
    }
}

#[test]
fn serialize_full() {
    let instant = Instant::now();
    let time = Utc::now();
    let doc = WeatherSettings {
        id: None,
        chat_id: String::from("123456"),
        location: Some(Location {
            latitude: 11.11,
            longitude: 11.11,
        }),
        time: Some(DateTime(time)),
    };

    let result = bson::to_document(&doc).unwrap();

    let location = doc.location.unwrap();
    let truncated_time = time.duration_trunc(Duration::milliseconds(1)).unwrap();
    let expected = doc! {
        "chat_id" : doc.chat_id,
        "location" : {
            "latitude" : location.latitude,
            "longitude" : location.longitude
        },
        "time" : truncated_time
    };
    assert_eq!(result, expected);
}

#[test]
fn serialize_small() {
    let doc = WeatherSettings {
        id: None,
        chat_id: String::from("123456"),
        location: None,
        time: None,
    };

    let result = bson::to_document(&doc).unwrap();

    let expected = doc! {
        "chat_id" : doc.chat_id,
    };
    assert_eq!(result, expected);
}
