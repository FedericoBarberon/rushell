#[allow(unused_imports)]
use std::io::{self, Write};

use codecrafters_shell::parser::ParsedInput;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read stdin");

        let parsed_input = ParsedInput::parse(&input);

        match parsed_input.command.as_str() {
            "exit" => break,
            "echo" => println!("{}", parsed_input.args.join(" ")),
            "" => continue,
            _ => println!("{}: command not found", &parsed_input.command),
        }
    }
}
