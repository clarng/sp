use std::error::Error;
use std::io;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt, Interest};

pub const SERVER_PORT: &str = "10294";

async fn handle_read(stream: &TcpStream) -> Result<(), Box<dyn Error>> {
    let mut data = vec![0; 1024];
    match stream.try_read(&mut data) {
        Ok(n) => {
            println!("read {} bytes", n);
        }
        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
        }
        Err(e) => {
            println!("error {}", e);
            return Err(e.into());
        }
    }
    Ok(())
}

async fn handle_write(stream: &TcpStream) -> Result<(), Box<dyn Error>> {
    match stream.try_write(b"Hello World") {
        Ok(n) => {
            println!("write {} bytes", n);
        }
        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
        }
        Err(e) => {
            println!("error {}", e);
            return Err(e.into());
        }
    }
    Ok(())
}

pub async fn main() -> Result<(), Box<dyn Error>> {
    let binding = format!("{address}:{port}", address="127.0.0.1", port=SERVER_PORT);
    let listener: TcpListener = TcpListener::bind(binding).await?;

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::spawn(async move {
            loop {
                let ready = stream.ready(Interest::READABLE | Interest::WRITABLE).await.unwrap();

                if ready.is_readable() {
                    println!("readable");
                    handle_read(&stream).await.unwrap();
                }

                if ready.is_writable() {
                    println!("writable");
                    handle_write(&stream).await.unwrap();
                }
            }
        });
    }
}
