use std::io::{Read, Write};

use crate::{
    commands::Command,
    execution::{Executable, ExecutionResult},
    parser::ParsedInput,
};

#[derive(Debug, PartialEq)]
pub struct TypeCmd {
    commands: Vec<String>,
}

impl TypeCmd {
    pub fn new(commands: Vec<String>) -> Self {
        Self { commands }
    }
}

impl Executable for TypeCmd {
    fn execute(
        &self,
        _input: &mut impl Read,
        output: &mut impl Write,
        _error: &mut impl Write,
    ) -> ExecutionResult {
        if self.commands.is_empty() {
            return ExecutionResult::Continue;
        }

        for cmd in &self.commands {
            match Command::try_from(ParsedInput::from(cmd.as_str())) {
                Ok(Command::External(ext)) => {
                    let _ = writeln!(output, "{cmd} is {}", ext.path().to_string_lossy());
                }
                Ok(_) => {
                    let _ = writeln!(output, "{cmd} is a shell builtin");
                }
                Err(_) => {
                    let _ = writeln!(output, "{cmd}: not found");
                }
            }
        }

        ExecutionResult::Continue
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        execution::ExecutionResult,
        tests::utilities::{EnvPathGuard, TestBuffers, build_args, create_executable},
    };
    use serial_test::serial;
    use tempfile::tempdir;

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
    #[serial]
    fn external_cmd() {
        let name = "my_command";
        let dir = tempdir().unwrap();
        let file = create_executable(name, "", dir.path());

        // SAFETY: the test has #[serial] macro
        let _guard = EnvPathGuard::prepend(dir.path());

        let TestBuffers {
            mut input,
            mut output,
            mut error,
        } = TestBuffers::new(None);
        let type_cmd = TypeCmd::new(build_args(&[name]));

        assert_eq!(
            type_cmd.execute(&mut input, &mut output, &mut error),
            ExecutionResult::Continue
        );

        assert_eq!(
            String::from_utf8(output).unwrap(),
            format!("{name} is {}\n", file.to_string_lossy())
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
