use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub enum Header {
    Client = 0b0000_0000,
    Server = 0b1111_1111,
}

impl TryFrom<u8> for Header {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b0000_0000 => Ok(Self::Client),
            0b1111_1111 => Ok(Self::Server),
            _ => Err("Unknown Header!!!"),
        }
    }
}
