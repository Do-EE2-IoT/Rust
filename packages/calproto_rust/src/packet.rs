use std::io::Error;

use crate::header::Header;
use crate::{client::Client, error::MyCalError};
use bytes::{Buf, BufMut, BytesMut};
use futures::{SinkExt, StreamExt};
use prost::Message;
use protocol::proto::{ClientMessage, ServerMessage};
use tokio_util::codec::{Decoder, Encoder, Framed};

#[derive(Clone, Debug)]
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
                dst.put_u8(Header::Client as u8);
                client_message.encode(&mut buf).unwrap();
                dst.extend_from_slice(&buf);
            }
            Packet::Server(server_message) => {
                let mut buf: Vec<u8> = Vec::new();
                dst.put_u8(Header::Server as u8);
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

        let header = src.get_u8();
        match Header::try_from(header) {
            Ok(header) => match header {
                Header::Client => {
                    if let Ok(cli_msg) = ClientMessage::decode(src) {
                        return Ok(Some(Packet::Client(cli_msg)));
                    }
                }
                Header::Server => {
                    if let Ok(server_msg) = ServerMessage::decode(src) {
                        return Ok(Some(Packet::Server(server_msg)));
                    }
                }
            },
            Err(e) => {
                println!("{e}")},
        }
        Err(MyCalError::DecodeError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::BytesMut;
    use protocol::proto::{
        client_message, server_message, Ack, Connect, Connack, Disconnect, ExpressionRequest,
        ExpressionResult, Ping,
    };

    #[test]
    fn test_encode_decode_connect_message() {
        let codec = &mut CalProtoCodec;
        let mut buf = BytesMut::new();

        let client_message = ClientMessage {
            payload: Some(client_message::Payload::Connect(Connect {
                client_id: "test_client".to_string(),
            })),
        };

        codec
            .encode(Packet::Client(client_message.clone()), &mut buf)
            .expect("Encoding failed");

        let decoded = codec
            .decode(&mut buf)
            .expect("Decoding failed")
            .expect("No message decoded");

        if let Packet::Client(decoded_message) = decoded {
            assert_eq!(decoded_message, client_message);
        } else {
            panic!("Decoded message is not a ClientMessage");
        }
    }

    #[test]
    fn test_encode_decode_expression_request() {
        let codec = &mut CalProtoCodec;
        let mut buf = BytesMut::new();

        let client_message = ClientMessage {
            payload: Some(client_message::Payload::ExpressionRequest(ExpressionRequest {
                client_id: "test_client".to_string(),
                message_id: 1.0,
                operand1: 10.0,
                operand2: 20.0,
                operand3: 30.0,
                operator1: "+".to_string(),
                operator2: "*".to_string(),
            })),
        };

        codec
            .encode(Packet::Client(client_message.clone()), &mut buf)
            .expect("Encoding failed");

        let decoded = codec
            .decode(&mut buf)
            .expect("Decoding failed")
            .expect("No message decoded");

        if let Packet::Client(decoded_message) = decoded {
            assert_eq!(decoded_message, client_message);
        } else {
            panic!("Decoded message is not a ClientMessage");
        }
    }

    #[test]
    fn test_encode_decode_expression_result() {
        let codec = &mut CalProtoCodec;
        let mut buf = BytesMut::new();

        let server_message = ServerMessage {
            payload: Some(server_message::Payload::ExpressionResult(ExpressionResult {
                message_id: 1.0,
                result: 42.0,
            })),
        };

        codec
            .encode(Packet::Server(server_message.clone()), &mut buf)
            .expect("Encoding failed");

        let decoded = codec
            .decode(&mut buf)
            .expect("Decoding failed")
            .expect("No message decoded");

        if let Packet::Server(decoded_message) = decoded {
            assert_eq!(decoded_message, server_message);
        } else {
            panic!("Decoded message is not a ServerMessage");
        }
    }

    #[test]
    fn test_encode_decode_disconnect_message() {
        let codec = &mut CalProtoCodec;
        let mut buf = BytesMut::new();

        let client_message = ClientMessage {
            payload: Some(client_message::Payload::Disconnect(Disconnect {
                client_id: "test_client".to_string(),
            })),
        };

        codec
            .encode(Packet::Client(client_message.clone()), &mut buf)
            .expect("Encoding failed");

        let decoded = codec
            .decode(&mut buf)
            .expect("Decoding failed")
            .expect("No message decoded");

        if let Packet::Client(decoded_message) = decoded {
            assert_eq!(decoded_message, client_message);
        } else {
            panic!("Decoded message is not a ClientMessage");
        }
    }

    #[test]
    fn test_encode_decode_ping_message() {
        let codec = &mut CalProtoCodec;
        let mut buf = BytesMut::new();

        let client_message = ClientMessage {
            payload: Some(client_message::Payload::Ping(Ping {
                client_id: "test_client".to_string(),
            })),
        };

        codec
            .encode(Packet::Client(client_message.clone()), &mut buf)
            .expect("Encoding failed");

        let decoded = codec
            .decode(&mut buf)
            .expect("Decoding failed")
            .expect("No message decoded");

        if let Packet::Client(decoded_message) = decoded {
            assert_eq!(decoded_message, client_message);
        } else {
            panic!("Decoded message is not a ClientMessage");
        }
    }
}
