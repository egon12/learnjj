use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub struct Client {
    conn: Option<TcpStream>,
}

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Str(String),
}

impl Into<std::io::Error> for Error {
    fn into(self) -> std::io::Error {
        match self {
            Error::Io(err) => err,
            Error::Str(s) => std::io::Error::new(std::io::ErrorKind::Other, s),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::Io(e)
    }
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
