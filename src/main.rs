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
                core::cat_to_num(
                    input.trim().to_string().split(" ").into_iter().map(|item| item.to_string()).collect(),
                    base4::alphabet(),
                    base4::char_length()
                )
            )
        } else if trimmed == "2".to_string() {
            input = "".to_string();
            stdin.read_line(&mut input).unwrap();
            println!(
                "{}",
                core::num_to_cat(
                    input.trim().parse().unwrap(),
                    base4::alphabet(),
                    base4::char_length()
                )
            )
        } else {
            println!("Invalid input, exiting...");
            break;
        }
        println!();
    }
}
