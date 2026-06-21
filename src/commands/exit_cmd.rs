use std::io::{Read, Write};

use crate::{commands::InvalidArgsError, execution::Executable};

#[derive(Debug, PartialEq)]
pub struct ExitCmd {
    code: i32,
}

impl ExitCmd {
    pub fn new(code: i32) -> Self {
        Self { code }
    }
}

impl TryFrom<&[String]> for ExitCmd {
    type Error = InvalidArgsError;

    fn try_from(value: &[String]) -> Result<Self, Self::Error> {
        match value {
            [] => Ok(Self::new(0)),
            [c] => {
                let code = c
                    .parse::<i32>()
                    .map_err(|_| InvalidArgsError::InvalidFormat { value: c.clone() })?;
                Ok(Self::new(code))
            }
            _ => Err(InvalidArgsError::TooManyArgs),
        }
    }
}

impl TryFrom<Vec<String>> for ExitCmd {
    type Error = InvalidArgsError;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        value.as_slice().try_into()
    }
}

impl Executable for ExitCmd {
    fn execute(
        &self,
        _input: &mut impl Read,
        _output: &mut impl Write,
        _error: &mut impl Write,
    ) -> crate::execution::ExecutionResult {
        crate::execution::ExecutionResult::Exit
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        execution::ExecutionResult,
        tests::utilities::{TestBuffers, build_args},
    };

    #[test]
    fn from_valid_args() {
        let args = build_args(&["1"]);
        let cmd = ExitCmd::try_from(args).unwrap();

        assert_eq!(cmd, ExitCmd::new(1));
    }

    #[test]
    fn from_empty_args() {
        let args = build_args(&[]);
        let cmd = ExitCmd::try_from(args).unwrap();

        assert_eq!(cmd, ExitCmd::new(0));
    }

    #[test]
    fn fails_from_multiple_args() {
        let args = build_args(&["2", "3"]);
        let err = ExitCmd::try_from(args).unwrap_err();

        assert_eq!(err, InvalidArgsError::TooManyArgs);
    }

    #[test]
    fn execute_retursn_exit_result() {
        let TestBuffers {
            mut input,
            mut output,
            mut error,
        } = TestBuffers::new(None);
        let exit = ExitCmd::new(0);

        assert_eq!(
            exit.execute(&mut input, &mut output, &mut error),
            ExecutionResult::Exit
        );

        assert!(output.is_empty());
        assert!(input.get_ref().is_empty());
        assert!(error.is_empty());
    }
}
