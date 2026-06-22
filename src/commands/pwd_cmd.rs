use std::{
    env,
    io::{Read, Write},
};

use crate::{
    commands::InvalidArgsError,
    execution::{Executable, ExecutionResult},
};

#[derive(Debug, PartialEq)]
pub struct PwdCmd {}

impl PwdCmd {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryFrom<&[String]> for PwdCmd {
    type Error = InvalidArgsError;

    fn try_from(value: &[String]) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Ok(Self::new())
        } else {
            Err(InvalidArgsError::TooManyArgs)
        }
    }
}

impl TryFrom<Vec<String>> for PwdCmd {
    type Error = InvalidArgsError;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        value.as_slice().try_into()
    }
}

impl Executable for PwdCmd {
    fn execute(
        &self,
        _input: &mut impl Read,
        output: &mut impl Write,
        error: &mut impl Write,
    ) -> ExecutionResult {
        let cwd = env::current_dir();

        let _ = match cwd {
            Ok(path) => writeln!(output, "{}", path.to_string_lossy()),
            Err(e) => writeln!(error, "Failed to get cwd: {e}"),
        };

        ExecutionResult::Continue
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use crate::{
        execution::ExecutionResult,
        tests::utilities::{TestBuffers, build_args},
    };

    #[test]
    fn from_empty_args() {
        let args = build_args(&[]);
        let cmd = PwdCmd::try_from(args).unwrap();

        assert_eq!(cmd, PwdCmd::new());
    }

    #[test]
    fn fails_from_multiple_args() {
        let args = build_args(&["foo"]);
        let err = PwdCmd::try_from(args).unwrap_err();

        assert_eq!(err, InvalidArgsError::TooManyArgs);

        let args = build_args(&["foo", "bar"]);
        let err = PwdCmd::try_from(args).unwrap_err();

        assert_eq!(err, InvalidArgsError::TooManyArgs);
    }

    #[test]
    fn execute_prints_cwd() {
        let TestBuffers {
            mut input,
            mut output,
            mut error,
        } = TestBuffers::new(None);
        let cmd = PwdCmd::new();

        assert_eq!(
            cmd.execute(&mut input, &mut output, &mut error),
            ExecutionResult::Continue
        );

        let cwd = env::current_dir().unwrap();
        let expected = format!("{}\n", cwd.to_string_lossy());

        assert_eq!(String::from_utf8_lossy(&output), expected);
        assert!(input.get_ref().is_empty());
        assert!(error.is_empty());
    }
}
