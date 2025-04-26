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
    Disconnect,
}

#[async_trait::async_trait]
pub trait Input {
    async fn pop(&mut self) -> io::Result<ConsoleInput>;
}

impl ConsoleInput {
    fn get_oprand_and_operator(op1: f64, op2: f64, op3: f64, oprt1: String, oprt2: String) -> Self {
        Self::Operand(ExpressionRequest {
            client_id: Default::default(),
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
            return Ok(Self::Disconnect);
        }

        if parts.len() != 5 {
            return Err(InputErr::InvalidFormat);
        }

        let operator1 = parts[1].to_string();
        let operator2 = parts[3].to_string();

        let valid_operators = ["+", "-", "*", "/"];
        if !valid_operators.contains(&operator1.as_str())
            || !valid_operators.contains(&operator2.as_str())
        {
            return Err(InputErr::InvalidOperator);
        }

        let op1 = match parts[0].parse::<f64>() {
            Ok(num) => num,
            Err(_) => return Err(InputErr::InvalidOperand),
        };

        let op2 = match parts[2].parse::<f64>() {
            Ok(num) => {
                if num == 0.0 && operator1 == "/"{
                    return Err(InputErr::Dividebyzero);
                }
                num
            }
            Err(_) => return Err(InputErr::InvalidOperand),
        };

        let op3 = match parts[4].parse::<f64>() {
            Ok(num) => {
                if num == 0.0 && operator2 == "/" {
                    return Err(InputErr::Dividebyzero);
                }
                num
            }
            Err(_) => return Err(InputErr::InvalidOperand),
        };



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

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::proto::ExpressionRequest;

    #[test]
    fn test_valid_expression_input() {
        let input = "10 + 20 * 30";
        let result = ConsoleInput::from_str(input).unwrap();

        if let ConsoleInput::Operand(expr) = result {
            assert_eq!(expr.operand1, 10.0);
            assert_eq!(expr.operator1, "+");
            assert_eq!(expr.operand2, 20.0);
            assert_eq!(expr.operator2, "*");
            assert_eq!(expr.operand3, 30.0);
        } else {
            panic!("Expected Operand variant");
        }
    }

    #[test]
    fn test_disconnect_input() {
        let input = "exit";
        let result = ConsoleInput::from_str(input).unwrap();
        assert!(matches!(result, ConsoleInput::Disconnect));
    }

    #[test]
    fn test_invalid_format() {
        let input = "10 + 20";
        let result = ConsoleInput::from_str(input);
        assert!(matches!(result, Err(InputErr::InvalidFormat)));
    }

    #[test]
    fn test_invalid_operator() {
        let input = "10 ^ 20 % 30";
        let result = ConsoleInput::from_str(input);
        assert!(matches!(result, Err(InputErr::InvalidOperator)));
    }

    #[test]
    fn test_invalid_operand() {
        let input = "10 + abc * 30";
        let result = ConsoleInput::from_str(input);
        assert!(matches!(result, Err(InputErr::InvalidOperand)));
    }

    #[test]
    fn test_divide_by_zero() {
        let input = "10 / 0 + 30";
        let result = ConsoleInput::from_str(input);
        assert!(matches!(result, Err(InputErr::Dividebyzero)));

        let input = "10 + 20 / 0";
        let result = ConsoleInput::from_str(input);
        assert!(matches!(result, Err(InputErr::Dividebyzero)));
    }
}
