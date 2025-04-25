use calproto_rust::server::Server;
use calproto_rust::{client::Client, packet::Packet};
use protocol::proto::{server_message, ClientMessage, ExpressionResult, ServerMessage};
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::{mpsc::Sender, Mutex};

async fn handle_client(mut server: Server, tx: Sender<Packet>, mut rx: Receiver<Packet>) {
    loop {
        tokio::select! {
            Ok(msg) = server.recv_from_client() =>  {
                println!("Recv something from client");
                if let Err(e) = tx.send(msg).await {
                    println!("Error send {:?}", e);
                }
            },

            Some(msg) = rx.recv() =>  {
               server.send_to_client(msg).await;

            }
        }
    }
}

async fn handle_logic(mut rx: Receiver<Packet>, tx: Sender<Packet>) {
    while let Some(msg) = rx.recv().await {
        match msg {
            calproto_rust::packet::Packet::Client(msg) => match msg.payload {
                Some(payload) => match payload {
                    protocol::proto::client_message::Payload::Ack(_) => todo!(),
                    protocol::proto::client_message::Payload::Connect(_) => {
                        println!("Connect")
                    }
                    protocol::proto::client_message::Payload::Disconnect(_) => {
                        println!("disconnect")
                    }
                    protocol::proto::client_message::Payload::ExpressionRequest(expressreq) => {
                        println!("Request from client !!!");
                        println!(
                            "{} {} {} {} {} ",
                            expressreq.operand1,
                            expressreq.operator1,
                            expressreq.operand2,
                            expressreq.operator2,
                            expressreq.operand3
                        );
                        let result_ =
                            match (expressreq.operator1.as_str(), expressreq.operator2.as_str()) {
                                ("+", "+") => {
                                    expressreq.operand1 + expressreq.operand2 + expressreq.operand3
                                }
                                ("+", "-") => {
                                    expressreq.operand1 + expressreq.operand2 - expressreq.operand3
                                }
                                ("+", "*") => {
                                    expressreq.operand1 + expressreq.operand2 * expressreq.operand3
                                }
                                ("+", "/") => {
                                    if expressreq.operand3 != 0.0 {
                                        expressreq.operand1
                                            + expressreq.operand2 / expressreq.operand3
                                    } else {
                                        println!("Division by zero error");
                                        0.0
                                    }
                                }
                                ("-", "+") => {
                                    expressreq.operand1 - expressreq.operand2 + expressreq.operand3
                                }
                                ("-", "-") => {
                                    expressreq.operand1 - expressreq.operand2 - expressreq.operand3
                                }
                                ("-", "*") => {
                                    expressreq.operand1 - expressreq.operand2 * expressreq.operand3
                                }
                                ("-", "/") => {
                                    if expressreq.operand3 != 0.0 {
                                        expressreq.operand1
                                            - expressreq.operand2 / expressreq.operand3
                                    } else {
                                        println!("Division by zero error");
                                        0.0
                                    }
                                }
                                ("*", "+") => {
                                    expressreq.operand1 * expressreq.operand2 + expressreq.operand3
                                }
                                ("*", "-") => {
                                    expressreq.operand1 * expressreq.operand2 - expressreq.operand3
                                }
                                ("*", "*") => {
                                    expressreq.operand1 * expressreq.operand2 * expressreq.operand3
                                }
                                ("*", "/") => {
                                    if expressreq.operand3 != 0.0 {
                                        expressreq.operand1 * expressreq.operand2
                                            / expressreq.operand3
                                    } else {
                                        println!("Division by zero error");
                                        0.0
                                    }
                                }
                                ("/", "+") => {
                                    if expressreq.operand2 != 0.0 {
                                        expressreq.operand1 / expressreq.operand2
                                            + expressreq.operand3
                                    } else {
                                        println!("Division by zero error");
                                        0.0
                                    }
                                }
                                ("/", "-") => {
                                    if expressreq.operand2 != 0.0 {
                                        expressreq.operand1 / expressreq.operand2
                                            - expressreq.operand3
                                    } else {
                                        println!("Division by zero error");
                                        0.0
                                    }
                                }
                                ("/", "*") => {
                                    if expressreq.operand2 != 0.0 {
                                        expressreq.operand1 / expressreq.operand2
                                            * expressreq.operand3
                                    } else {
                                        println!("Division by zero error");
                                        0.0
                                    }
                                }
                                ("/", "/") => {
                                    if expressreq.operand2 != 0.0 && expressreq.operand3 != 0.0 {
                                        expressreq.operand1
                                            / expressreq.operand2
                                            / expressreq.operand3
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
                            payload: Some(server_message::Payload::ExpressionResult(
                                ExpressionResult {
                                    message_id: expressreq.message_id,
                                    result: result_,
                                },
                            )),
                        };
                        if let Err(e) = tx.send(Packet::Server(msg)).await {
                            println!(" Error {:?}", e);
                        }
                    }
                    protocol::proto::client_message::Payload::Ping(ping_msg) => {
                        println!("Get Ping msg from {}", ping_msg.client_id)
                    }
                },
                None => todo!(),
            },
            _ => println!("Hello world"),
        }
    }
}

#[tokio::main]
async fn main() {
    let server = Server::config((127, 0, 0, 1), 7878).await;
    loop {
        let (tx_client, rx_logic): (Sender<Packet>, Receiver<Packet>) =
            tokio::sync::mpsc::channel(10);
        let (tx_logic, rx_client): (Sender<Packet>, Receiver<Packet>) =
            tokio::sync::mpsc::channel(10);
        tokio::spawn(async move { handle_client(server, tx_client, rx_logic).await });
        tokio::spawn(async move { handle_logic(rx_client, tx_logic).await });
    }
}


// https://gist.github.com/Do-EE2-IoT/d8abdd02ec88fc57f266460fd7549cf3