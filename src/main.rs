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
        let mut tokens = input.split_whitespace();

        let Some(command) = tokens.next() else {
            continue;
        };

        let args = tokens.collect::<Vec<&str>>();

        match command {
            "exit" => break,
            "echo" => println!("{}", args.join(" ")),
            _ => println!("{input}: command not found"),
        }
    }
}
