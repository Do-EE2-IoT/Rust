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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_try_from_valid_values() {
        assert_eq!(Header::try_from(0b0000_0000), Ok(Header::Client));
        assert_eq!(Header::try_from(0b1111_1111), Ok(Header::Server));
    }

    #[test]
    fn test_header_try_from_invalid_value() {
        assert_eq!(Header::try_from(0b1010_1010), Err("Unknown Header!!!"));
    }

    #[test]
    fn test_header_enum_values() {
        assert_eq!(Header::Client as u8, 0b0000_0000);
        assert_eq!(Header::Server as u8, 0b1111_1111);
    }
}
