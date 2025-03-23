use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878")
        .await
        .expect("Failed to bind address");

    println!("Server listening on port 7878...");
    let (mut stream, socket) = listener
        .accept()
        .await
        .expect("Failed to accept connection");
    let addr: std::net::IpAddr = socket.ip();
    match addr {
        std::net::IpAddr::V4(v4_addr) => {
            println!("Client connected with IPv4 address: {}", v4_addr);
        }
        std::net::IpAddr::V6(v6_addr) => {
            println!("Client connected with IPv6 address: {}", v6_addr);
        }
    }
    let mut buffer: [u8; 1024] = [0; 1024];
    loop {
        match stream.read(&mut buffer).await {
            Ok(_) => {
                println!("Received message: {}", String::from_utf8_lossy(&buffer));
                if let Err(e) = stream.write_all("Hello client \n".as_bytes()).await {
                    println!("Error {e} when response");
                }
            }
            Err(e) => eprintln!("Failed to read from stream: {}", e),
        }
    }
}
