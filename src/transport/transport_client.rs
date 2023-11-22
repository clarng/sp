use tokio::net::TcpStream;

pub async fn new_client(host: &str, port: &str) -> TcpStream {
    let address = &format!("{address}:{port}", address=host, port=port);
    let stream = TcpStream::connect(address).await.unwrap();
    return stream
}