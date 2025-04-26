use calproto_rust::client::Client;
use calproto_rust::packet::Packet;
use protocol::proto::{client_message, ClientMessage, Connect, ServerMessage};
use stdin_kb::stdin::{Console, ConsoleInput, Input};
use tokio::sync::mpsc::{Receiver, Sender};

async fn console_input_handle(tx: Sender<ConsoleInput>) {
    let mut console_input = Console::default();
    while let Ok(in_data) = console_input.pop().await {
        if let Err(e) = tx.send(in_data).await {
            println!("Send input channel err: {:?}", e);
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, mut rx): (Sender<ConsoleInput>, Receiver<ConsoleInput>) =
        tokio::sync::mpsc::channel(10);

    tokio::spawn(console_input_handle(tx));
    let mut client = Client::config((127, 0, 0, 1), 7878, "Do mixi gaming".to_string()).await;

    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3));
    let mut message_id_count: f64 = 0.0;

    'retry: for attempt in 1..=3 {
        let connect_msg = Packet::Client(ClientMessage {
            payload: Some(client_message::Payload::Connect(Connect {
                client_id: client.client_id.clone(),
            })),
        });
        if let Err(e) = client.send_to_server(connect_msg).await{

            println!( " Error when send message to server {:?}", e);
        }

        match tokio::time::timeout(tokio::time::Duration::from_secs(30), client.recv_from_server()).await {
            Ok(Ok(Packet::Server(msg))) => match msg.payload {
                Some(payload) => match payload {
                    protocol::proto::server_message::Payload::Connack(msg) => {
                        println!("Server accepted connection, new ID: {}", msg.client_id);
                        client.client_id = msg.client_id;
                        break;
                    }
                    _ => println!("Must be a Connack message"),
                },
                None => todo!(),
            },
            Ok(Ok(_)) => println!("Don't care"),
            Ok(Err(e)) => {
                println!("Error receiving response from server: {:?}", e);
                break 'retry;
            }
            Err(_) => {
                println!("Timeout waiting for server response");
                break 'retry;
            }
        }

        if attempt == 3 {
            println!("Failed to connect after 3 attempts. Exiting program.");
            return;
        } else {
            println!("Retrying connection (attempt {}/{})...", attempt, 3);
        }
    }

    loop {
        tokio::select! {
            _ = interval.tick() => {
                println!("Prepare send Ping to server");
                let msg = ClientMessage {
                    payload: Some(client_message::Payload::Ping(
                        protocol::proto::Ping { client_id: client.client_id.clone() }
                    ))
                };
                if let Err(e) = client.send_to_server(Packet::Client(msg)).await{
                    println!("Error when send message to server: {:?}", e)
                }
            },

            Some(input) = rx.recv() => {
                println!("Get some thing");
                match input {
                    ConsoleInput::Disconnect => {
                        let msg = ClientMessage {
                            payload: Some(client_message::Payload::Disconnect(
                                protocol::proto::Disconnect { client_id: client.client_id.clone() }
                            ))
                        };
                        if let Err(e) = client.send_to_server(Packet::Client(msg)).await{
                            println!("Error when send message to server: {:?}", e)
                        }
                    }
                    ConsoleInput::Operand(exreq) => {
                        let msg = ClientMessage {
                            payload: Some(client_message::Payload::ExpressionRequest({
                                protocol::proto::ExpressionRequest {
                                    client_id: client.client_id.clone(),
                                    message_id: message_id_count,
                                    operand1: exreq.operand1,
                                    operand2: exreq.operand2,
                                    operand3: exreq.operand3,
                                    operator1: exreq.operator1,
                                    operator2: exreq.operator2,
                                }
                            })),
                        };
                        message_id_count += 1.0;
                        if let Err(e) = client.send_to_server(Packet::Client(msg)).await{
                            println!("Error when send message to server: {:?}", e)
                        }
                    }
                }
            },

            Ok(msg) = client.recv_from_server() => {
                match msg {
                    Packet::Server(msg) => match msg.payload {
                        Some(payload) => match payload {
                            protocol::proto::server_message::Payload::ExpressionResult(expresult) => {
                                println!(
                                    "We have message id {} with result {}",
                                    expresult.message_id, expresult.result
                                )
                            }
                            protocol::proto::server_message::Payload::Ack(ack) => {
                                println!("Get ACK {}", ack.message_id)
                            }
                            _ => println!("Dont care"),
                        },
                        None => todo!(),
                    },
                    _ => println!("Unknown"),
                }
            }
        }
    }
}
