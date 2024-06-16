use crate::errors::ConversionError;

pub fn parse(decimal: &str) -> Result<i64, ConversionError> {
    decimal
        .parse::<i64>()
        .map_err(|_| ConversionError::InvalidNumber)
}

pub fn from_decimal(decimal: i64) -> String {
    format!("@dec -> {}", decimal)
}
