mod client;

use client::Client;
use client::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut c = Client::new();
    c.connect("[::1]:8083".to_string()).await?;
    c.write("hello".to_string()).await?;

    Ok(())
}
