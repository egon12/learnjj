use crate::error::Error;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub struct Client {
    conn: Option<TcpStream>,
}

impl Client {
    pub fn new() -> Self {
        Client { conn: None }
    }

    pub async fn connect(&mut self, addr: String) -> Result<(), Error> {
        let conn = TcpStream::connect(addr).await?;
        self.conn = Some(conn);
        Ok(())
    }

    pub async fn write(&mut self, data: String) -> Result<(), Error> {
        self.conn
            .as_mut()
            .ok_or(Error::Str("no connection".to_string()))?
            .write(data.as_bytes())
            .await?;
        Ok(())
    }
}
