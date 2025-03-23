use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let client = TcpStream::connect("127.0.0.1:7878").await;
    let mut stream = if let Ok(client_stream) = client {
        client_stream
    } else {
        println!("Failed to connect to server!");
        return;
    };
    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader).lines();
    loop {
        tokio::select! {
            _ = sleep(Duration::from_secs(1))
             => {
                if let Err(e) = writer.write_all("Hello server !".as_bytes()).await {
                    println!("Failed to write to server, error: {e}");
                }
                println!("Send message to server!");

            },

            _ = async {
                match reader.next_line().await {
                    Ok(Some(line)) => {
                        println!("Received from server: {}", line);
                    },
                    Ok(None) => {
                        println!("Server closed the connection.");
                    },
                    Err(e) => {
                        println!("Failed to read from server, error: {e}");
                    }
                }
            } => {},
        }
    }
}
