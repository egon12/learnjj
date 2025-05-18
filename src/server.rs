use tokio::{io::AsyncReadExt, net::TcpListener};

use crate::error::Error;

pub struct Server {
    listener: Option<TcpListener>,
}

impl Server {
    pub fn new() -> Self {
        Server { listener: None }
    }

    pub async fn listen(&mut self, addr: String) -> Result<(), Error> {
        eprintln!("bind");
        let listener = TcpListener::bind(addr).await?;
        self.listener = Some(listener);
        eprintln!("bind done");
        Ok(())
    }

    pub async fn run_blocked(&mut self) -> Result<(), Error> {
        let listener = self
            .listener
            .as_mut()
            .ok_or(Error::Str("cannot get the listeners".to_string()))?;

        eprintln!("start accept");
        let (mut conn, addr) = listener.accept().await?;
        eprintln!("accept done");
        eprintln!("{:?}", addr);

        loop {
            let mut buf = [0; 4096];
            let n = conn.read(&mut buf[..]).await?;

            if n < 1 {
                eprintln!("n < 1");
                break;
            }

            let s: String = String::from_utf8(buf[0..n].to_vec())?;

            println!("{}", s)
        }

        Ok(())
    }
}
