use mongodb::Client;
use mongodb::options::ClientOptions;
use tokio;

pub async fn db_connect() {
    todo!();
}

#[tokio::test]
pub async fn connect() {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options);
    assert!(client.is_ok(), "Failed to create MongoDB Client");
}
