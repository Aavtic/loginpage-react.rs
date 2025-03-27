use mongodb::Client;
use mongodb::options::ClientOptions;

pub struct MongoClient {
    client: Client,
}

impl MongoClient {
    pub async fn connect() -> MongoClient {
        let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
        let client = Client::with_options(client_options).unwrap();

        return MongoClient {
            client
        };
    }

    pub async fn create_collection(&self, db_name: &str, collection_name: &str, ) {
        let db = &self.client.database(db_name);
        db.create_collection(collection_name).await.unwrap();
    }
}
