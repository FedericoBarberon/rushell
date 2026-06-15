use crate::parser::ParsedInput;

#[derive(Debug, PartialEq)]
pub enum ExecutionResult {
    Continue,
    Exit,
}

pub fn execute(parsed_input: ParsedInput) -> ExecutionResult {
    match parsed_input.command.as_str() {
        "exit" => ExecutionResult::Exit,
        "echo" => {
            println!("{}", parsed_input.args.join(" "));
            ExecutionResult::Continue
        }
        "" => ExecutionResult::Continue,
        _ => {
            println!("{}: command not found", &parsed_input.command);
            ExecutionResult::Continue
        }
    }
}
