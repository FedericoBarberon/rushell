use std::io::{Read, Write};

use crate::execution::Executable;

#[derive(Debug, PartialEq)]
pub struct ExitCmd {}

impl ExitCmd {
    pub fn new() -> Self {
        Self {}
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
    use crate::{execution::ExecutionResult, tests::utilities::TestBuffers};

    use super::*;

    #[test]
    fn test_exit() {
        let TestBuffers {
            mut input,
            mut output,
            mut error,
        } = TestBuffers::new(None);
        let exit = ExitCmd::new();

        assert_eq!(
            exit.execute(&mut input, &mut output, &mut error),
            ExecutionResult::Exit
        );

        assert!(output.is_empty());
        assert!(input.get_ref().is_empty());
        assert!(error.is_empty());
    }
}
