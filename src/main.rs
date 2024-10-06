use std::{env, process};

mod converters;
mod errors;

use crate::converters::{binary, decimal, hexadecimal};
use crate::errors::ConversionError;

fn main() {
    let args: Vec<String> = env::args().collect();

    // // bdh -in b
    // // bdh -out b
    // // bdh -in b -out h
    if args.len() != 3 {
        eprintln!("Example: bdh -h 3FF");
        process::exit(1);
    }

    let from = &args[1];
    let value = &args[2];

    match convert(from, value) {
        Ok(r) => print(r),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn convert(from: &str, value: &str) -> Result<i64, ConversionError> {
    let decimal = match from {
        "-b" => binary::to_decimal(value)?,
        "-d" => decimal::parse(value)?,
        "-h" => hexadecimal::to_decimal(value)?,
        _ => return Err(ConversionError::InvalidFormat),
    };
    Ok(decimal)
}

fn print(decimal: i64) {
    let binary_number = binary::from_decimal(decimal);

    println!(
        "({}) bits",
        binary_number
            .chars()
            .filter(|&c| c == '1' || c == '0')
            .count()
    );
    println!("{}", binary_number);
    println!("{}", decimal::from_decimal(decimal));
    println!("{}", hexadecimal::from_decimal(decimal));
}
