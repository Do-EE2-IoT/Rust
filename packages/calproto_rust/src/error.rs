#[derive(Debug)]
pub enum MyCalError {
    InvalidHeader(u8),
    DecodeError,
    EncodeError,
    ConnectionError,
    GetAckError,
    RecvMessageError,
    SendMessageError,
    IoError(std::io::Error),
}

impl From<std::io::Error> for MyCalError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}
