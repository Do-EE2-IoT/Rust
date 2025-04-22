use crate::err::InputErr;
use protocol::proto::{Disconnect, ExpressionRequest};
use std::{f64::consts::E, io, str::FromStr};
use tokio::io::{AsyncBufReadExt, BufReader};

#[derive(Default)]
pub struct Console {
    pub buffer: String,
}

#[derive(Debug)]
pub enum ConsoleInput {
    Operand(ExpressionRequest),
    Disconnect(Disconnect),
}

#[async_trait::async_trait]
pub trait Input {
    async fn pop(&mut self) -> io::Result<ConsoleInput>;
}

impl ConsoleInput {
    fn get_oprand_and_operator(op1: f64, op2: f64, op3: f64, oprt1: String, oprt2: String) -> Self {
        Self::Operand(ExpressionRequest {
            header: Default::default(),
            message_id: Default::default(),
            operand1: op1,
            operand2: op2,
            operand3: op3,
            operator1: oprt1,
            operator2: oprt2,
        })
    }
}
impl FromStr for ConsoleInput {
    type Err = InputErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        if parts.is_empty() {
            return Err(InputErr::InputEmty);
        }

        if parts.len() == 1 && parts[0].to_lowercase() == "exit" {
            return Ok(Self::Disconnect(Disconnect {
                header: Default::default(),
                client_id: Default::default(),
            }));
        }

        if parts.len() != 5 {
            return Err(InputErr::InvalidFormat);
        }

        let op1 = match parts[0].parse::<f64>() {
            Ok(num) => num,
            Err(_) => return Err(InputErr::InvalidOperand),
        };

        let op2 = match parts[2].parse::<f64>() {
            Ok(num) => num,
            Err(_) => return Err(InputErr::InvalidOperand),
        };

        let op3 = match parts[4].parse::<f64>() {
            Ok(num) => num,
            Err(_) => return Err(InputErr::InvalidOperand),
        };

        // Get operators
        let operator1 = parts[1].to_string();
        let operator2 = parts[3].to_string();

        // Validate operators (assuming we allow +, -, *, /, ^)
        let valid_operators = ["+", "-", "*", "/"];
        if !valid_operators.contains(&operator1.as_str())
            || !valid_operators.contains(&operator2.as_str())
        {
            return Err(InputErr::InvalidOperator);
        }

        Ok(Self::get_oprand_and_operator(
            op1, op2, op3, operator1, operator2,
        ))
    }
}

#[async_trait::async_trait]
impl Input for Console {
    async fn pop(&mut self) -> io::Result<ConsoleInput> {
        let stdin = tokio::io::stdin();
        let mut reader = BufReader::new(stdin);
        loop {
            self.buffer.clear();
            match reader.read_line(&mut self.buffer).await {
                Ok(_) => match ConsoleInput::from_str(self.buffer.trim_end()) {
                    Ok(item) => {
                        break Ok(item);
                    }
                    Err(err) => {
                        println!("Error --- {:?}", err);
                    }
                },
                Err(err) => {
                    println!("Error reading input: {}", err);
                }
            }
        }
    }
}
