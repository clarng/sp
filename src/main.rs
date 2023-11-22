use std::env;

use tokio::{net::TcpStream, runtime::Runtime, io::AsyncWriteExt};

mod transport;

#[tokio::main]
#[allow(dead_code)]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        println!("Supply either s or c");
        return
    }
    let mode = &args[1];
    if mode == "s" {
        println!("Server mode");
        transport::transport::main().await.unwrap();
    } else {
        println!("Client mode");
        let mut client = transport::transport_client::new_client("127.0.0.1", transport::transport::SERVER_PORT).await;

        let data = vec![0u8; 100_000_000]; // 10 MB of zeros for example
        client.write_all(&data).await.unwrap();

        println!("Data sent successfully!");
    }
    
}