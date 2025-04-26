use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::{error::MyCalError, packet::Packet};
use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use crate::packet::CalProtoCodec;


pub struct ServerListener{
    listener: TcpListener,
}

impl ServerListener{
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
        Self{
            listener
        }

    }

    pub async fn accept_connect(&mut self) -> Option<Server>{
        if let Ok (connect,) = self.listener.accept().await{
            let framed = Framed::new(connect.0, CalProtoCodec);
            println!("Accept connect with CLIENT IP : {}", connect.1);
            Some(Server { framed })
        }else{
            None
        }
    
    }
}
pub struct Server {
    framed: Framed<TcpStream, CalProtoCodec>,
}

impl Server {
    pub async fn send_to_client(&mut self, packet: Packet) -> Result<(), MyCalError>{
        if let Err(e) = self.framed.send(packet).await {
            Err(MyCalError::SendMessageError)
        }else{
            Ok(())
        }
    }

    pub async fn recv_from_client(&mut self) -> Result<Packet, MyCalError> {
        match self.framed.next().await {
            Some(packet) => match packet {
                Ok(msg) => Ok(msg),
                Err(e) => {
                    Err(e)
                },
            },
            None => {
                Err(MyCalError::RecvMessageError)},
        }
    }
}
