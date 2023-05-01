use mongodb::{Client, options::ClientOptions};
use std::env;
use dotenv::dotenv;
use lazy_static::lazy_static;

pub struct MyDatabase(pub mongodb::Database);

lazy_static! {
    static ref MONGODB_URI: String = env::var("MONGO_DB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    static ref MONGODB_DATABASE_NAME: String = env::var("MONGO_DB_NAME").unwrap_or_else(|_| "my_database".to_string());
}

pub async fn establish_connection() -> MyDatabase {
    dotenv().ok();

    let client_options = ClientOptions::parse(&*MONGODB_URI).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    MyDatabase(client.database(&*MONGODB_DATABASE_NAME))
}
