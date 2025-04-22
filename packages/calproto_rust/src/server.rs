use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::{error::MyCalError, packet::Packet};
use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use crate::packet::CalProtoCodec;

pub struct Server {
    framed: Framed<TcpStream, CalProtoCodec>,
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
        let framed = Framed::new(stream, CalProtoCodec);
        Self { framed }
    }

    pub async fn send_to_server(&mut self, packet: Packet) {
        if let Err(e) = self.framed.send(packet).await {
            println!("Error {:?}", e);
        }
    }

    pub async fn recv_from_server(&mut self) -> Result<Packet, MyCalError> {
        match self.framed.next().await {
            Some(packet) => match packet {
                Ok(msg) => Ok(msg),
                Err(e) => Err(e),
            },
            None => Err(MyCalError::RecvMessageError),
        }
    }
}
