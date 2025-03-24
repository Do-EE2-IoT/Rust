use crate::error::MyCalError;
enum MyCalHeader {
    Connect = 0x01,
    Disconnect = 0x02,
    Ping = 0x03,
    Ack = 0x04,
    ExpressionRequest = 0x05,
    ExpressionResult = 0x06,
}



impl TryFrom<u8> for MyCalHeader {
    type Error = MyCalError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(MyCalHeader::Connect),
            0x02 => Ok(MyCalHeader::Disconnect),
            0x03 => Ok(MyCalHeader::Ping),
            0x04 => Ok(MyCalHeader::Ack),
            0x05 => Ok(MyCalHeader::ExpressionRequest),
            0x06 => Ok(MyCalHeader::ExpressionResult),
            _ => Err(MyCalError::InvalidHeader(value)),
        }
    }
}
