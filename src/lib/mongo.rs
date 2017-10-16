use std;
use std::env;

use mongodb;
use mongodb::{Client, ThreadedClient};

const DEFAULT_MONGO_ADDRESS: &'static str = "127.0.0.1";

pub fn establish_connection() -> std::sync::Arc<mongodb::ClientInner> {
  let database_url = match env::var("MONGO_ADDRESS") {
    Ok(value) => value,
    Err(_) => DEFAULT_MONGO_ADDRESS.to_string(),
  };

  let client = Client::connect(&database_url, 27017)
                .expect("Failed to initialize standalone client.");

  client
}
