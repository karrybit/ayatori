use crate::token_type::TokenType;

#[derive(Default)]
pub(crate) struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    examining_char: Option<char>,
}

impl Lexer {
    pub(crate) fn new(input: String) -> Self {
        let mut lexer = Self {
            input: input.chars().collect(),
            ..Default::default()
        };
        lexer.read();
        lexer
    }
    pub(crate) fn next(&mut self) -> TokenType {
        self.skip_whitespace();
        match self.examining_char {
            Some(ch) if ch == '=' => {
                self.read();
                TokenType::Equal
            }
            Some(ch) if ch == '{' => {
                self.read();
                TokenType::LBrace
            }
            Some(ch) if ch == '}' => {
                self.read();
                TokenType::RBrace
            }
            Some(ch) if Self::is_letter(&ch) => {
                let literal = self.read_identifier();
                TokenType::Literal(literal)
            }
            Some(ch) => TokenType::Illegal(ch),
            None => TokenType::EOF,
        }
    }
    fn read(&mut self) {
        self.examining_char = self.input.get(self.read_position).cloned();
        self.position = self.read_position;
        self.read_position += 1;
    }
    fn skip_whitespace(&mut self) {
        while self
            .examining_char
            .as_ref()
            .map_or(false, char::is_ascii_whitespace)
        {
            self.read();
        }
    }
    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.examining_char.as_ref().map_or(false, Self::is_letter) {
            self.read();
        }
        self.input[position..self.position]
            .iter()
            .collect::<String>()
            .as_str()
            .trim_matches('"')
            .to_string()
    }
    fn is_letter(ch: &char) -> bool {
        ch.is_ascii_alphanumeric()
            || ch == &'_'
            || ch == &'"'
            || ch == &':'
            || ch == &'-'
            || ch == &'.'
    }
}
