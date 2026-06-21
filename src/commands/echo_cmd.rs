use std::io::{Read, Write};

use crate::execution::Executable;

#[derive(Debug, PartialEq)]
pub struct EchoCmd {
    args: Vec<String>,
}

impl EchoCmd {
    pub fn new(args: Vec<String>) -> Self {
        Self { args }
    }
}

impl Executable for EchoCmd {
    fn execute(
        &self,
        _input: &mut impl Read,
        output: &mut impl Write,
        _error: &mut impl Write,
    ) -> crate::execution::ExecutionResult {
        let _ = writeln!(output, "{}", self.args.join(" "));
        crate::execution::ExecutionResult::Continue
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
    fn execution_writes_to_output() {
        let TestBuffers {
            mut input,
            mut output,
            mut error,
        } = TestBuffers::new(None);
        let echo = EchoCmd::new(build_args(&["foo", "bar"]));

        assert_eq!(
            echo.execute(&mut input, &mut output, &mut error),
            ExecutionResult::Continue
        );

        assert_eq!(String::from_utf8(output).unwrap(), "foo bar\n");
        assert!(input.get_ref().is_empty());
        assert!(error.is_empty());
    }

    #[test]
    fn execution_with_no_args() {
        let TestBuffers {
            mut input,
            mut output,
            mut error,
        } = TestBuffers::new(None);
        let echo = EchoCmd::new(Vec::new());

        assert_eq!(
            echo.execute(&mut input, &mut output, &mut error),
            ExecutionResult::Continue
        );

        assert_eq!(String::from_utf8(output).unwrap(), "\n");
        assert!(input.get_ref().is_empty());
        assert!(error.is_empty());
    }
}
