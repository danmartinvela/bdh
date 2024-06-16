use std::fmt;

#[derive(Debug)]
pub enum ConversionError {
    InvalidNumber,
    InvalidFormat,
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConversionError::InvalidNumber => write!(f, "Unvalid number"),
            ConversionError::InvalidFormat => write!(f, "Unvalid format"),
        }
    }
}

impl std::error::Error for ConversionError {}
