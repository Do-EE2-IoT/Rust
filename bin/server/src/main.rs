use calproto_rust::server::Server;
use protocol::proto::ClientMessage;
#[tokio::main]
async fn main() {
    let mut server = Server::config((127, 0, 0, 1), 7878).await;

    while let Ok(msg) = server.recv_from_client().await {
        println!("Get some message from client ?//");
        match msg {
            calproto_rust::packet::Packet::Client(msg) => match msg.payload {
                Some(payload) => match payload {
                    protocol::proto::client_message::Payload::Ack(_) => todo!(),
                    protocol::proto::client_message::Payload::Connect(_) => println!("Connect"),
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
                        let result =
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
                        println!("Result: {}", result);
                    }
                    protocol::proto::client_message::Payload::Ping(_) => println!("Get Ping"),
                },
                None => todo!(),
            },
            _ => todo!(),
        }
    }
}
