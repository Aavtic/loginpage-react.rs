use mongodb::Client;
use mongodb::options::ClientOptions;
use mongodb::results::UpdateResult;
use bson::doc;

use crate::types::UserCredential;


#[derive(Clone)]
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

// Functions for checking stuff in the collection.
impl MongoClient {
    /// Checks if user exists and return true otherwise return false.
    pub async fn check_user_exists(&self, db_name: &str, coll: &str,  username: &str) -> bool {
        let collection = &self.client.database(db_name).collection::<UserCredential>(coll);
        let result = collection.find_one(doc! {"username": username}).await;
        if let Ok(Some(_username)) = result {
            return true;
        }
        return false;
    }

    pub async fn insert_user(&self, db_name: &str, coll: &str, user_creds: UserCredential) {
        let collection = &self.client.database(db_name).collection::<UserCredential>(coll);
        // TODO: Handls ts.
        collection.insert_one(user_creds).await.unwrap();
    }
}

// Functions for retrieving stuff from the collection
impl MongoClient {
    /// Function to get password from a collection of type `UserCredential` as defined in types.rs.
    /// TODO: Make this more generic.
    pub async fn get_password(&self, db_name: &str, coll: &str,  username: &str) -> Option<String> {
        let collection = &self.client.database(db_name).collection::<UserCredential>(coll);
        let result = collection.find_one(doc! {"username": username}).await;
        if let Ok(Some(creds)) = result {
            return Some(creds.password);
        } else {
            return None;
        }
    }
}

// User Session related Functions.

impl MongoClient {
    pub async fn add_session_key(&self, db_name: &str, coll: &str, session_key: &str, username: &str) -> UpdateResult {
        let collection = &self.client.database(db_name).collection::<UserCredential>(coll);
        // Unwraps because It's sure that the only logical call to this function will be
        // after a user account is created. 
        // Anyway if necessary the unwraps will be handled.
        let mut session_pool = collection.find_one(doc! {"username": username}).await.unwrap().unwrap().session_key_pool;
        session_pool.push(session_key.to_string());
        let filter = doc!{"username": username};
        let update = doc!{"$set": doc!{"session_key_pool": session_pool}};

        let result = collection.update_one(filter, update).await.unwrap();
        return result;
    }
}
