use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use tokio::net::TcpStream;
use tokio_util::codec::Framed;

struct MyCodec;
pub struct Client {
    framed: Framed<TcpStream, MyCodec>,
    client_id: String,
}

impl Client {
    pub async fn config(server_ip: (u8, u8, u8, u8), server_port: u16, client_id: String) -> Self {
        let ip = IpAddr::V4(Ipv4Addr::new(
            server_ip.0,
            server_ip.1,
            server_ip.2,
            server_ip.3,
        ));
        let socket_addr = SocketAddr::new(ip, server_port);

        let stream = TcpStream::connect(socket_addr)
            .await
            .expect("Must create stream with server");

        let framed = Framed::new(stream, MyCodec);

        Self { framed, client_id }
    }
    pub async fn send(&mut self) {
        todo!();
    }

    pub async fn wait_message(&mut self) {
        todo!();
    }
}
