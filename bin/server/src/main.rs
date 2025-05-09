use calproto_rust::server::{Server, ServerListener};
use calproto_rust::{client::Client, packet::Packet};
use protocol::proto::{server_message, ClientMessage, ExpressionResult, ServerMessage};
use tokio::sync::mpsc::{Receiver, Sender};
static mut CLIENT_ID_COUNT: u32 = 0;
async fn handle_client(mut server: Server, tx: Sender<Packet>, mut rx: Receiver<Packet>) {
    loop {
        tokio::select! {
            result = server.recv_from_client() => {
                match result {
                    Ok(msg) => {
                        if let Err(e) = tx.send(msg).await {
                            println!("Error sending to logic handler: {:?}", e);
                        }
                    },
                    Err(_) => {}
                }
            },

            Some(msg) = rx.recv() => {
                if let Err(e) = server.send_to_client(msg).await {
                    println!("Error sending to client: {:?}", e);
                }
            },

            else => {
                println!("All channels have been closed");
                break;
            }
        }
    }
    println!("Client handler terminated");
}

async fn handle_logic(mut rx: Receiver<Packet>, tx: Sender<Packet>) {
    while let Some(msg) = rx.recv().await {
        match msg {
            Packet::Client(msg) => match msg.payload {
                Some(payload) => match payload {
                    protocol::proto::client_message::Payload::Ack(_) => todo!(),
                    protocol::proto::client_message::Payload::Connect(msg) => {
                        let msg = Packet::Server(ServerMessage {
                            payload: Some(server_message::Payload::Connack(protocol::proto::Connack {
                                client_id: format!("{}__{}", msg.client_id,unsafe {
                                    CLIENT_ID_COUNT
                                } ),
                            })),
                        });
                       unsafe {
                           CLIENT_ID_COUNT += 1;
                       }

                        if let Err(e) = tx.send(msg).await {
                            println!("Error: {:?}", e);
                        }
                    }
                    protocol::proto::client_message::Payload::Disconnect(_) => {
                        println!("Disconnect");
                    }
                    protocol::proto::client_message::Payload::ExpressionRequest(expressreq) => {
                        println!("Request from client !!!");
                        println!(
                            "{} {} {} {} {}",
                            expressreq.operand1,
                            expressreq.operator1,
                            expressreq.operand2,
                            expressreq.operator2,
                            expressreq.operand3
                        );

                        let result_ = match (expressreq.operator1.as_str(), expressreq.operator2.as_str()) {
                            ("+", "+") => expressreq.operand1 + expressreq.operand2 + expressreq.operand3,
                            ("+", "-") => expressreq.operand1 + expressreq.operand2 - expressreq.operand3,
                            ("+", "*") => expressreq.operand1 + expressreq.operand2 * expressreq.operand3,
                            ("+", "/") => {
                                if expressreq.operand3 != 0.0 {
                                    expressreq.operand1 + expressreq.operand2 / expressreq.operand3
                                } else {
                                    println!("Division by zero error");
                                    0.0
                                }
                            }
                            ("-", "+") => expressreq.operand1 - expressreq.operand2 + expressreq.operand3,
                            ("-", "-") => expressreq.operand1 - expressreq.operand2 - expressreq.operand3,
                            ("-", "*") => expressreq.operand1 - expressreq.operand2 * expressreq.operand3,
                            ("-", "/") => {
                                if expressreq.operand3 != 0.0 {
                                    expressreq.operand1 - expressreq.operand2 / expressreq.operand3
                                } else {
                                    println!("Division by zero error");
                                    0.0
                                }
                            }
                            ("*", "+") => expressreq.operand1 * expressreq.operand2 + expressreq.operand3,
                            ("*", "-") => expressreq.operand1 * expressreq.operand2 - expressreq.operand3,
                            ("*", "*") => expressreq.operand1 * expressreq.operand2 * expressreq.operand3,
                            ("*", "/") => {
                                if expressreq.operand3 != 0.0 {
                                    expressreq.operand1 * expressreq.operand2 / expressreq.operand3
                                } else {
                                    println!("Division by zero error");
                                    0.0
                                }
                            }
                            ("/", "+") => {
                                if expressreq.operand2 != 0.0 {
                                    expressreq.operand1 / expressreq.operand2 + expressreq.operand3
                                } else {
                                    println!("Division by zero error");
                                    0.0
                                }
                            }
                            ("/", "-") => {
                                if expressreq.operand2 != 0.0 {
                                    expressreq.operand1 / expressreq.operand2 - expressreq.operand3
                                } else {
                                    println!("Division by zero error");
                                    0.0
                                }
                            }
                            ("/", "*") => {
                                if expressreq.operand2 != 0.0 {
                                    expressreq.operand1 / expressreq.operand2 * expressreq.operand3
                                } else {
                                    println!("Division by zero error");
                                    0.0
                                }
                            }
                            ("/", "/") => {
                                if expressreq.operand2 != 0.0 && expressreq.operand3 != 0.0 {
                                    expressreq.operand1 / expressreq.operand2 / expressreq.operand3
                                } else {
                                    println!("Division by zero error");
                                    0.0
                                }
                            }
                            _ => {
                                println!("Unsupported operators");
                                0.0
                            }
                        };

                        println!("Result: {}", result_);

                        let msg = ServerMessage {
                            payload: Some(server_message::Payload::ExpressionResult(ExpressionResult {
                                message_id: expressreq.message_id,
                                result: result_,
                            })),
                        };

                        if let Err(e) = tx.send(Packet::Server(msg)).await {
                            println!("Error: {:?}", e);
                        }
                    }
                    protocol::proto::client_message::Payload::Ping(ping_msg) => {
                        println!("Get Ping msg from {}", ping_msg.client_id);
                    }
                },
                None => println!("Unknown message"),
            },
            _ => println!("Hello world"),
        }
    }
}

#[tokio::main]
async fn main() {
    let mut server_config = ServerListener::config((0, 0, 0, 0), 7878).await;

    while let Some(server) = server_config.accept_connect().await {
        let (tx_client, rx_client): (Sender<Packet>, Receiver<Packet>) = tokio::sync::mpsc::channel(10);
        let (tx_logic, rx_logic): (Sender<Packet>, Receiver<Packet>) = tokio::sync::mpsc::channel(10);

        tokio::spawn(handle_client(server, tx_client, rx_logic));
        tokio::spawn(handle_logic(rx_client, tx_logic));
    }
}