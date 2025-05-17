use tokio::io::Error;

mod client;

use client::Client;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut c = Client::new();
    c.connect("[::1]:8083".to_string()).await?;

    Ok(())
}
