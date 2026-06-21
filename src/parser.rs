#[derive(Debug, PartialEq)]
pub struct ParsedInput {
    pub command: String,
    pub args: Vec<String>,
}

impl ParsedInput {
    pub fn new(command: String, args: Vec<String>) -> Self {
        Self { command, args }
    }
}

impl From<&str> for ParsedInput {
    fn from(value: &str) -> Self {
        let value = value.trim();
        let mut tokens = value.split_whitespace();

        let command = tokens.next().unwrap_or("").to_owned();
        let args = tokens.map(String::from).collect::<Vec<String>>();

        Self { command, args }
    }
}

impl From<String> for ParsedInput {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::utilities::build_args;

    use super::*;

    #[test]
    fn command_with_multiple_args() {
        test_eq("cmd foo bar", "cmd", &["foo", "bar"]);
    }

    #[test]
    fn command_with_one_arg() {
        test_eq("cmd foo", "cmd", &["foo"]);
    }

    #[test]
    fn command_without_args() {
        test_eq("cmd", "cmd", &[]);
    }

    #[test]
    fn empty_input() {
        test_eq("", "", &[]);
    }

    #[test]
    fn trim_whitespaces() {
        test_eq("   cmd    foo     bar   ", "cmd", &["foo", "bar"]);
    }

    fn test_eq(input: &str, cmd: &str, args: &[&str]) {
        let expected = ParsedInput::new(cmd.to_owned(), build_args(args));

        assert_eq!(ParsedInput::from(input), expected);
    }
}
