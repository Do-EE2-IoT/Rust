use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::sleep;
use tokio_util::codec::{Decoder, Encoder, Framed};
use bytes::{BufMut, BytesMut};
use std::io;
use futures::{SinkExt, StreamExt};
use prost::Message;

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
async fn main() {
    let stream = match TcpStream::connect("127.0.0.1:7878").await {
        Ok(s) => s,
        Err(_) => {
            println!("Failed to connect to server!");
            return;
        }
    };

    let mut framed = Framed::new(stream, MyCodec);

    loop {
        tokio::select! {
            _ = sleep(Duration::from_secs(1)) => {
                let message = MyMessage { id: 1, content: "Hello server!".to_string() };
                if let Err(e) = framed.send(message).await {
                    println!("Failed to write to server, error: {}", e);
                }
                println!("Sent message to server!");
            },
            Some(Ok(received)) = framed.next() => {
                println!("Received from server: {:?}", received);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::BytesMut;

    #[test]
    fn test_encode_decode() {
        let mut codec = MyCodec;
        let message = MyMessage { id: 42, content: "Test message".to_string() };
        let mut dst = BytesMut::new();

        // Encode the message
        codec.encode(message.clone(), &mut dst).expect("Encoding failed");

        // Decode the message
        let decoded = codec.decode(&mut dst).expect("Decoding failed").expect("No message decoded");

        // Assert that the original and decoded messages are the same
        assert_eq!(decoded, message, "Decoded message does not match the original");
    }
}
