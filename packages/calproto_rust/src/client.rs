use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::packet::CalProtoCodec;
use crate::{error::MyCalError, packet::Packet};
use futures::{SinkExt, StreamExt};
use protocol::proto::{ClientMessage, ServerMessage};
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

pub struct Client {
    framed: Framed<TcpStream, CalProtoCodec>,
    pub client_id: String,
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

        let framed = Framed::new(stream, CalProtoCodec);

        Self { framed, client_id }
    }
    pub async fn send_to_server(&mut self, packet: Packet) {
        if let Err(e) = self.framed.send(packet).await {
            println!("Error {:?} when client send!", e);
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
