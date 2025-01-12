extern crate cat2text;
use cat2text::{base4, core};
use std::{io, process::exit};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        println!("Pick your translation:");
        println!("1) cat to text");
        println!("2) text to cat");
        input = "".to_string();
        stdin.read_line(&mut input).unwrap();
        let trimmed = input.trim();
        if trimmed == "1".to_string() {
            input = "".to_string();
            stdin.read_line(&mut input).unwrap();
            println!(
                "{}",
                base4::decode(
                    input.trim().to_string()
                    )
            );
        } else if trimmed == "2".to_string() {
            input = "".to_string();
            stdin.read_line(&mut input).unwrap();
            println!(
                "{}",
                base4::encode(input.trim().to_string())
                
            );
        } else {
            println!("Invalid input, exiting...");
            break;
        }
        println!();
    }
}
