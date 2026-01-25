use std::sync::Arc;

use redis_clone::server::Server;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::Notify,
};

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let ready = Arc::new(Notify::new());
    let ready_for_server = ready.clone();
    let ready_for_client = ready;

    let addr = "127.0.0.1:8080".to_string();
    let stream_addr = addr.clone();

    let server = Server::new(addr);

    tokio::spawn(async move {
        server.listen_with_signal(ready_for_server).await.unwrap();
    });

    ready_for_client.notified().await;

    let mut stream = TcpStream::connect(stream_addr).await?;
    stream.write_all(b"Hello from client").await?;

    let mut response = [0; 1024];
    let n = stream.read(&mut response).await?;
    println!(
        "[Client] Server response : {}",
        String::from_utf8_lossy(&response[..n])
    );

    Ok(())
}
