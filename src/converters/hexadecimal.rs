use crate::errors::ConversionError;

pub fn to_decimal(hex: &str) -> Result<i64, ConversionError> {
    i64::from_str_radix(hex, 16).map_err(|_| ConversionError::InvalidNumber)
}

pub fn from_decimal(decimal: i64) -> String {
    format!("@hex -> {:X}", decimal)
}
