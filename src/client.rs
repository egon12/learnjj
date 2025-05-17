use tokio::io::Error;
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
}
