use crate::errors::ConversionError;

pub fn to_decimal(binary: &str) -> Result<i64, ConversionError> {
    i64::from_str_radix(binary, 2).map_err(|_| ConversionError::InvalidNumber)
}

pub fn from_decimal(decimal: i64) -> String {
    format!("@bin -> {:b}", decimal)
}
