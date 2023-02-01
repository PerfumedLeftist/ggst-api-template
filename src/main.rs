mod ggst_api;
mod requests;
mod responses;
use ggst_api::*;
use tokio;
use dotenv::dotenv;

use std::env;
#[tokio::main]
async fn main() {
    match dotenv() {
        Ok(_) => println!("Sucessfully loaded environment variables."),
        Err(err) => {
            println!("Error parsing environment variables:\n{}", err);
            panic!();
        }
    }
    let replays = get_replays().await.unwrap();
    println!("{:?}", replays)
}
