use std::io::{Read, Write};

use crate::{
    commands::{
        echo_cmd::EchoCmd, exit_cmd::ExitCmd, external_cmd::ExternalCmd, type_cmd::TypeCmd,
    },
    execution::Executable,
    finder::find_executable_in_path,
    parser::ParsedInput,
};

mod echo_cmd;
mod exit_cmd;
mod external_cmd;
mod type_cmd;

#[derive(Debug, PartialEq)]
pub enum Command {
    Exit(ExitCmd),
    Echo(EchoCmd),
    Type(TypeCmd),
    External(ExternalCmd),
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
            Command::External(cmd) => cmd.execute(input, output, error),
        }
    }
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum CommandParseError {
    #[error("{cmd}: command not found")]
    UnknownCommand { cmd: String },
    #[error(transparent)]
    InvalidArgs(#[from] InvalidArgsError),
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum InvalidArgsError {
    #[error("too many arguments")]
    TooManyArgs,
    #[error("too few arguments")]
    TooFewArgs,
    #[error("{value}: invalid format")]
    InvalidFormat { value: String },
}

impl TryFrom<ParsedInput> for Command {
    type Error = CommandParseError;

    fn try_from(value: ParsedInput) -> Result<Self, Self::Error> {
        match value.command.as_str() {
            "exit" => {
                let cmd =
                    ExitCmd::try_from(value.args).map_err(|e| CommandParseError::InvalidArgs(e))?;
                Ok(Command::Exit(cmd))
            }
            "echo" => Ok(Command::Echo(EchoCmd::new(value.args))),
            "type" => Ok(Command::Type(TypeCmd::new(value.args))),
            _ => {
                if let Some(path) = find_executable_in_path(&value.command) {
                    Ok(Command::External(ExternalCmd::new(
                        value.command,
                        path,
                        value.args,
                    )))
                } else {
                    Err(CommandParseError::UnknownCommand { cmd: value.command })
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;
    use tempfile::tempdir;

    use super::*;
    use crate::{
        parser::ParsedInput,
        tests::utilities::{EnvPathGuard, build_args, create_executable},
    };

    #[test]
    fn exit_cmd_from_parsed_input() {
        let parsed_input = ParsedInput::from("exit 1");
        let expected = Command::Exit(ExitCmd::new(1));

        assert_eq!(Command::try_from(parsed_input).unwrap(), expected);
    }

    #[test]
    fn exit_cmd_invalid_args_from_parsed_input() {
        let parsed_input = ParsedInput::from("exit foo");
        let expected_err = CommandParseError::InvalidArgs(InvalidArgsError::InvalidFormat {
            value: "foo".into(),
        });

        assert_eq!(Command::try_from(parsed_input).unwrap_err(), expected_err);
    }

    #[test]
    fn echo_cmd_from_parsed_input() {
        let parsed_input = ParsedInput::from("echo hello world");
        let expected = Command::Echo(EchoCmd::new(build_args(&["hello", "world"])));

        assert_eq!(Command::try_from(parsed_input).unwrap(), expected);
    }

    #[test]
    fn type_cmd_from_parsed_input() {
        let parsed_input = ParsedInput::from("type echo ls");
        let expected = Command::Type(TypeCmd::new(build_args(&["echo", "ls"])));

        assert_eq!(Command::try_from(parsed_input).unwrap(), expected);
    }

    #[test]
    #[serial]
    fn external_cmd() {
        let dir = tempdir().unwrap();
        let name = "my_command";
        let file = create_executable(name, "", dir.path());

        // SAFETY: the test has #[serial] macro
        let _guard = EnvPathGuard::prepend(dir.path());

        let parsed_input = ParsedInput::from("my_command foo bar");
        let expected_command = Command::External(ExternalCmd::new(
            name.into(),
            file,
            build_args(&["foo", "bar"]),
        ));

        let cmd = Command::try_from(parsed_input).unwrap();
        assert_eq!(cmd, expected_command);
    }

    #[test]
    fn unknown_command() {
        let cmd = "unknown_command";
        let parsed_input = ParsedInput::from(cmd);
        assert_eq!(
            Command::try_from(parsed_input).unwrap_err(),
            CommandParseError::UnknownCommand {
                cmd: cmd.to_owned()
            }
        )
    }
}
