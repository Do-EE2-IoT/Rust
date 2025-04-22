use std::io::Error;

use crate::{client::Client, error::MyCalError};
use bytes::{BufMut, BytesMut};
use futures::{SinkExt, StreamExt};
use prost::Message;
use protocol::proto::{ClientMessage, ServerMessage};
use tokio_util::codec::{Decoder, Encoder, Framed};
pub enum Packet {
    Client(ClientMessage),
    Server(ServerMessage),
}

pub struct CalProtoCodec;
impl Encoder<Packet> for CalProtoCodec {
    type Error = MyCalError;

    fn encode(&mut self, item: Packet, dst: &mut BytesMut) -> Result<(), Self::Error> {
        match item {
            Packet::Client(client_message) => {
                let mut buf: Vec<u8> = Vec::new();
                client_message.encode(&mut buf).unwrap();
                dst.extend_from_slice(&buf);
            }
            Packet::Server(server_message) => {
                let mut buf: Vec<u8> = Vec::new();
                server_message.encode(&mut buf).unwrap();
                dst.extend_from_slice(&buf);
            }
        }
        Ok(())
    }
}

impl Decoder for CalProtoCodec {
    type Item = Packet;

    type Error = MyCalError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() {
            return Err(MyCalError::DecodeError);
        }
        if let Ok(cli_msg) = ClientMessage::decode(src.clone()) {
            return Ok(Some(Packet::Client(cli_msg)));
        } else if let Ok(server_msg) = ServerMessage::decode(src) {
            return Ok(Some(Packet::Server(server_msg)));
        }
        Err(MyCalError::DecodeError)
    }
}
