use mongodb::Client;
use mongodb::options::ClientOptions;

pub async fn db_connect() {
    todo!();
}

pub async fn connect(mongo_addr: &str) -> Client {
    let client_options = ClientOptions::parse(mongo_addr).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    return client;
}
