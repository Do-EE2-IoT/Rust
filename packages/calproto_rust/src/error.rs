#[derive(Debug)]
pub enum MyCalError {
    InvalidHeader(u8),
    DecodeError,
    EncodeError,
    ConnectionError,
    GetAckError,
}