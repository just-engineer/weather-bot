use mongodb::bson;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::database::Documented;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ClientMessage {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    chat_id: String,
    message: String,
}

impl ClientMessage {
    pub fn new(chat_id: &str, message: &str) -> Self {
        ClientMessage {
            id: None,
            message: String::from(message),
            chat_id: String::from(chat_id),
        }
    }
}

impl Documented for ClientMessage {
    const COLLECTION_NAME: &'static str = "client_message";

    fn set_id(self, id: ObjectId) -> Self {
        ClientMessage {
            id: Some(id),
            message: self.message,
            chat_id: self.chat_id,
        }
    }
}

#[test]
fn bson_test() {
    let result = bson::to_document(&ClientMessage::new("1234", "message")).unwrap();
    eprintln!("result = {}", result);
}

