#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read stdin");

        let input = input.trim();

        if input == "exit" {
            break;
        }

        println!("{input}: command not found");
    }
}
