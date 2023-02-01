mod ggst_api;
mod requests;
mod responses;
use ggst_api::*;
use tokio;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let replays = get_replays().await.unwrap();
    println!("{:?}", replays)
}
