use std::sync::Arc;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::Notify,
};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        return Server { addr };
    }

    pub async fn listen_with_signal(&self, signal: Arc<Notify>) -> tokio::io::Result<()> {
        let listener = TcpListener::bind(&self.addr).await?;

        signal.notify_one();

        loop {
            let (mut socket, _) = listener.accept().await?;
            tokio::spawn(async move {
                let mut buf = [0; 1024];
                let n = socket.read(&mut buf).await.unwrap();
                println!("Received message : {}", String::from_utf8_lossy(&buf[..n]));
                socket
                    .write_all("awesome-message".as_bytes())
                    .await
                    .unwrap();
            });
        }
    }
}
