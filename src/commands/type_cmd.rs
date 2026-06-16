use std::io::{Read, Write};

use crate::{
    commands::Command,
    execution::{Executable, ExecutionResult},
    parser::ParsedInput,
};

#[derive(Debug, PartialEq)]
pub struct TypeCmd {
    args: Vec<String>,
}

impl TypeCmd {
    pub fn new(args: Vec<String>) -> Self {
        Self { args }
    }
}

impl Executable for TypeCmd {
    fn execute(
        &self,
        _input: &mut impl Read,
        output: &mut impl Write,
        _error: &mut impl Write,
    ) -> ExecutionResult {
        if self.args.is_empty() {
            return ExecutionResult::Continue;
        }

        for cmd in &self.args {
            if Command::try_from(ParsedInput::from(cmd.as_str())).is_ok() {
                let _ = writeln!(output, "{} is a shell builtin", cmd);
            } else {
                let _ = writeln!(output, "{}: not found", cmd);
            }
        }

        ExecutionResult::Continue
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        execution::ExecutionResult,
        tests::utilities::{TestBuffers, build_args},
    };

    use super::*;

    #[test]
    fn unknown_cmd() {
        let TestBuffers {
            mut input,
            mut output,
            mut error,
        } = TestBuffers::new(None);
        let type_cmd = TypeCmd::new(build_args(&["unknown_command"]));

        assert_eq!(
            type_cmd.execute(&mut input, &mut output, &mut error),
            ExecutionResult::Continue
        );

        assert_eq!(
            String::from_utf8(output).unwrap(),
            "unknown_command: not found\n"
        );
        assert!(input.get_ref().is_empty());
        assert!(error.is_empty());
    }

    #[test]
    fn built_in_cmd() {
        let TestBuffers {
            mut input,
            mut output,
            mut error,
        } = TestBuffers::new(None);
        let type_cmd = TypeCmd::new(build_args(&["echo"]));

        assert_eq!(
            type_cmd.execute(&mut input, &mut output, &mut error),
            ExecutionResult::Continue
        );

        assert_eq!(
            String::from_utf8(output).unwrap(),
            "echo is a shell builtin\n"
        );
        assert!(input.get_ref().is_empty());
        assert!(error.is_empty());
    }

    #[test]
    fn multiple_cmds() {
        let TestBuffers {
            mut input,
            mut output,
            mut error,
        } = TestBuffers::new(None);
        let type_cmd = TypeCmd::new(build_args(&["echo", "exit", "unknown_command", "type"]));

        assert_eq!(
            type_cmd.execute(&mut input, &mut output, &mut error),
            ExecutionResult::Continue
        );

        assert_eq!(
            String::from_utf8(output).unwrap(),
            "echo is a shell builtin\nexit is a shell builtin\nunknown_command: not found\ntype is a shell builtin\n"
        );
        assert!(input.get_ref().is_empty());
        assert!(error.is_empty());
    }
}
