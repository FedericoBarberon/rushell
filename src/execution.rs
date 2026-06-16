use std::io::{Read, Write};

#[derive(Debug, PartialEq)]
pub enum ExecutionResult {
    Continue,
    Exit,
}

pub trait Executable {
    fn execute(
        &self,
        input: &mut impl Read,
        output: &mut impl Write,
        error: &mut impl Write,
    ) -> ExecutionResult;
}
