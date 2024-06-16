use std::{env, process};

mod converters;
mod errors;

use crate::converters::{binary, decimal, hexadecimal};
use crate::errors::ConversionError;

fn main() {
    let args: Vec<String> = env::args().collect();

    // let args = &args[1..];

    // let mut from: &str;
    // let mut to: &str;
    // let mut value: &str;

    // for arg in args {
    //     if arg == "-in" || arg == "-out" {
    //         set_env(arg);
    //         break;
    //     } else {
    //     }
    // }

    // // bdh -in b
    // // bdh -out b
    // // bdh -in b -out h
    // if args.len() != 3 {
    //     eprintln!("Example: bdh -bd 1111");
    //     process::exit(1);
    // }

    let from = &args[1][0..2];
    let to = &args[1][2..];
    let value = &args[2];

    // match from {
    //     "-in" => set_in(),
    //     "-out" => set_out(),
    // }

    for c in to.chars() {
        match convert(from, c, value) {
            Ok(r) => println!("{}", r),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

fn convert(from: &str, to: char, value: &str) -> Result<String, ConversionError> {
    let decimal_value = match from {
        "-b" => binary::to_decimal(value)?,
        "-d" => decimal::parse(value)?,
        "-h" => hexadecimal::to_decimal(value)?,
        _ => return Err(ConversionError::InvalidFormat),
    };

    let result = match to {
        'b' => binary::from_decimal(decimal_value),
        'd' => decimal::from_decimal(decimal_value),
        'h' => hexadecimal::from_decimal(decimal_value),
        _ => return Err(ConversionError::InvalidFormat),
    };

    Ok(result)
}

// fn set_env(value: &str) {
//     match value {
//         "-in" => env::set_var("BDH_IN", value),
//         "-out" => env::set_var("BDH_OUT", value),
//         _ => unreachable!(),
//     }
// }
