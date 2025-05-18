mod client;
mod error;
mod server;

use client::Client;
use error::Error;
use server::Server;

use tokio::task;
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() -> Result<(), Error> {
    listen();

    sleep(Duration::from_secs(2)).await;

    let mut c = Client::new();
    c.connect("[::1]:8083".to_string()).await?;
    c.write("hello is there something in there\n".to_string())
        .await?;

    Ok(())
}

fn listen() {
    task::spawn(async {
        let mut s = Server::new();
        s.listen("[::1]:8083".to_string()).await.unwrap();
        s.run_blocked().await.unwrap();
    });
}
