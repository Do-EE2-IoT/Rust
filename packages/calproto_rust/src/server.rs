use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::Framed;

struct MyCodec;

pub struct Server {
    framed: Framed<TcpStream, MyCodec>,
}

impl Server {
    pub async fn config(server_ip: (u8, u8, u8, u8), server_port: u16) -> Self {
        let ip = IpAddr::V4(Ipv4Addr::new(
            server_ip.0,
            server_ip.1,
            server_ip.2,
            server_ip.3,
        ));
        let socket_addr = SocketAddr::new(ip, server_port);

        let listener = TcpListener::bind(socket_addr)
            .await
            .expect("Must bind listener to address");
        let (stream, socket) = listener.accept().await.expect("?");
        println!("accept connnect with client  {}", socket.ip());
        let framed = Framed::new(stream, MyCodec);
        Self { framed }
    }

    pub async fn wait_message_from_client(&mut self) {
        todo!();
    }

    pub async fn respond(&mut self) {
        todo!();
    }
}
