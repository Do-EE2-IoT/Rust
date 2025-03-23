use tokio::net::TcpListener;
use tokio_util::codec::{Decoder, Encoder, Framed};
use bytes::{BufMut, BytesMut};
use std::io;
use futures::SinkExt;
use prost::Message;
use futures::{ StreamExt};

#[derive(Clone, PartialEq, prost::Message)]
pub struct MyMessage {
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[prost(string, tag = "2")]
    pub content: String,
}

pub struct MyCodec;

impl Decoder for MyCodec {
    type Item = MyMessage;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() {
            return Ok(None);
        }
        MyMessage::decode(src).map(Some).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Decode error"))
    }
}

impl Encoder<MyMessage> for MyCodec {
    type Error = io::Error;

    fn encode(&mut self, item: MyMessage, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let mut buf = Vec::new();
        item.encode(&mut buf).map_err(|_| io::Error::new(io::ErrorKind::Other, "Encode error"))?;
        dst.put_slice(&buf);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    println!("Server listening on 127.0.0.1:7878");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New client connected: {}", addr);

        tokio::spawn(async move {
            let mut framed = Framed::new(socket, MyCodec);

            while let Some(Ok(msg)) = framed.next().await {
                println!("Received from client {}: {:?}", addr, msg);
                
                // Echo back the received message
                if let Err(e) = framed.send(msg.clone()).await {
                    println!("Failed to send response to {}: {}", addr, e);
                    break;
                }
            }
            println!("Client {} disconnected", addr);
        });
    }
}