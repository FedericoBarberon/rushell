use std::io::{Read, Write};

use crate::{
    commands::{echo_cmd::EchoCmd, exit_cmd::ExitCmd, type_cmd::TypeCmd},
    execution::Executable,
    parser::ParsedInput,
};

mod echo_cmd;
mod exit_cmd;
mod type_cmd;

#[derive(Debug, PartialEq)]
pub enum Command {
    Exit(ExitCmd),
    Echo(EchoCmd),
    Type(TypeCmd),
}

impl Executable for Command {
    fn execute(
        &self,
        input: &mut impl Read,
        output: &mut impl Write,
        error: &mut impl Write,
    ) -> crate::execution::ExecutionResult {
        match self {
            Command::Exit(cmd) => cmd.execute(input, output, error),
            Command::Echo(cmd) => cmd.execute(input, output, error),
            Command::Type(cmd) => cmd.execute(input, output, error),
        }
    }
}

#[derive(Debug, thiserror::Error, PartialEq)]
#[error("{cmd}: command not found")]
pub struct UnknownCommand {
    cmd: String,
}

impl TryFrom<ParsedInput> for Command {
    type Error = UnknownCommand;

    fn try_from(value: ParsedInput) -> Result<Self, Self::Error> {
        match value.command.as_str() {
            "exit" => Ok(Command::Exit(ExitCmd::new())),
            "echo" => Ok(Command::Echo(EchoCmd::new(value.args))),
            "type" => Ok(Command::Type(TypeCmd::new(value.args))),
            _ => Err(UnknownCommand { cmd: value.command }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parser::ParsedInput, tests::utilities::build_args};

    #[test]
    fn cmd_from_parsed_input() {
        let parsed_inputs = [ParsedInput::from("exit"), ParsedInput::from("echo foo bar")];
        let expected_commands = [
            Command::Exit(ExitCmd::new()),
            Command::Echo(EchoCmd::new(build_args(&["foo", "bar"]))),
        ];

        for (parsed_input, expected_command) in parsed_inputs.into_iter().zip(expected_commands) {
            let cmd = Command::try_from(parsed_input).unwrap();
            assert_eq!(cmd, expected_command)
        }
    }

    #[test]
    fn unknown_command() {
        let cmd = "unknown_command";
        let parsed_input = ParsedInput::from(cmd);
        assert_eq!(
            Command::try_from(parsed_input).unwrap_err(),
            UnknownCommand {
                cmd: cmd.to_owned()
            }
        )
    }
}
