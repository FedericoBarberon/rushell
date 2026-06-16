#[allow(unused_imports)]
use std::io::{self, Write};

use codecrafters_shell::{
    commands::Command,
    execution::{Executable, ExecutionResult},
    parser::ParsedInput,
};

fn main() {
    let mut input_buf = io::stdin();
    let mut output_buf = io::stdout();
    let mut error_buf = io::stderr();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read stdin");

        let parsed_input = ParsedInput::from(input);
        let command = Command::try_from(parsed_input);

        if let Err(e) = command {
            let _ = writeln!(error_buf, "{e}");
            continue;
        }

        let command = command.unwrap();

        match command.execute(&mut input_buf, &mut output_buf, &mut error_buf) {
            ExecutionResult::Continue => continue,
            ExecutionResult::Exit => break,
        }
    }
}
