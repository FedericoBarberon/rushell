#[allow(unused_imports)]
use std::io::{self, Write};

use codecrafters_shell::{
    executor::{ExecutionResult, execute},
    parser::ParsedInput,
};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read stdin");

        let parsed_input = ParsedInput::parse(&input);

        match execute(parsed_input) {
            ExecutionResult::Continue => continue,
            ExecutionResult::Exit => break,
        }
    }
}
