use anyhow::{Context, Result};
use clap::Parser;

/// Convert string to bytes array and back
#[derive(Parser)]
#[command(version)]
struct Cli {
    /// String or bytes array to convert
    thing: String,
}

fn main() {
    let args = Cli::parse();
    match run(args.thing) {
        Ok(s) => println!("{}", s),
        Err(e) => {
            eprintln!("ERROR {:#}", e);
            std::process::exit(1);
        }
    }
}

fn run(thing: String) -> Result<String> {
    if thing.chars().next().unwrap() == '[' {
        return bytes_to_string(thing);
    } else {
        return Ok(string_to_bytes(thing));
    }
}

fn string_to_bytes(s: String) -> String {
    format!("{:?}", s.as_bytes())
}

fn bytes_to_string(b: String) -> Result<String> {
    if b.len() < 2 {
        return Err(anyhow::anyhow!("Invalid bytes array"));
    }
    if b.chars().next().unwrap() != '[' || b.chars().last().unwrap() != ']' {
        return Err(anyhow::anyhow!("Invalid bytes array"));
    }
    let b = &b[1..b.len() - 1];
    if b.contains("0x") {
        // Hexadecimal
        let bytes: Vec<u8> = b
            .split(", ")
            .map(|x| u8::from_str_radix(&x[2..], 16).unwrap())
            .collect();
        return Ok(String::from_utf8(bytes).context("")?);
    } else {
        // Decimal
        let bytes: Vec<u8> = b
        .split(", ")
        .map(|x| x.parse().unwrap())
        .collect();
        return Ok(String::from_utf8(bytes).context("")?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_bytes() {
        assert_eq!(string_to_bytes("ABC".to_string()), "[65, 66, 67]");
    }

    #[test]
    fn test_bytes_to_string() -> Result<()> {
        assert_eq!(bytes_to_string("[65, 66, 67]".to_string())?, "ABC");
        Ok(())
    }

    #[test]
    fn test_bytes_hexa_to_string() -> Result<()> {
        assert_eq!(bytes_to_string("[0x41, 0x42, 0x43]".to_string())?, "ABC");
        Ok(())
    }
}
