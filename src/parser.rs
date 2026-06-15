#[derive(Debug, PartialEq)]
pub struct ParsedInput {
    pub command: String,
    pub args: Vec<String>,
}

impl ParsedInput {
    pub fn new(command: String, args: Vec<String>) -> Self {
        Self { command, args }
    }

    pub fn parse(input: &str) -> Self {
        let input = input.trim();
        let mut tokens = input.split_whitespace();

        let command = tokens.next().unwrap_or("").to_owned();
        let args = tokens.map(String::from).collect::<Vec<String>>();

        Self { command, args }
    }
}
