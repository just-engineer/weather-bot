use mongodb::{Database, Client, bson::{Document, Bson, oid::ObjectId}, bson};
use serde::Serialize;
use mongodb::options::{UpdateModifications, UpdateOptions, FindOneAndUpdateOptions};
use bson::doc;
use crate::client_message::ClientMessage;

#[derive(Debug, Clone)]
pub struct Datastore {
    database: Database,
}

impl Datastore {
    pub async fn init(mongo_uri: &str) -> Self {
        let client = Client::with_uri_str(mongo_uri).await.expect("wrong uri");
        let database = client.database("weather-bot");
        Datastore {
            database
        }
    }

    pub async fn upsert<T: Documented>(&self, query: Document, entity: T) -> T {
        let doc = entity.to_document();

        let collection = self.database.collection(T::COLLECTION_NAME);

        let update_options = UpdateOptions::builder().upsert(Some(true)).build();
        let update_options = FindOneAndUpdateOptions::builder().upsert(Some(true)).build();

        let result = collection
            .find_one_and_update(query, UpdateModifications::Document(doc), update_options).await
            .expect("something wrong");

        // if let Some(Bson::ObjectId(object_id)) = result.upserted_id {
        //     entity.set_id(object_id)
        // } else {
        //     entity
        // }

        entity
    }
}

pub trait Documented: Serialize {
    const COLLECTION_NAME: &'static str;

    fn to_document(&self) -> Document {
        bson::to_document(&self).unwrap()
    }

    fn set_id(self, id: ObjectId) -> Self;

    // todo запилить список индексов и колбек для создания на старте
}


#[tokio::test]
async fn update() {


}