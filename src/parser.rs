use crate::model::{Resource, ResourceType};
use crate::token_type::TokenType;
use std::collections::HashMap;

pub(crate) fn parse(content: String) -> Vec<Resource> {
    if content.is_empty() {
        return vec![];
    }
    let mut lexer = Lexer::new(content);
    let mut parser = Parser::new(&mut lexer);
    let resources = parser.parse_resources();
    resources
}

#[derive(Default)]
struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    examining_char: Option<char>,
}

impl Lexer {
    fn new(input: String) -> Self {
        let mut lexer = Self {
            input: input.chars().collect(),
            ..Default::default()
        };
        lexer.read();
        lexer
    }
    fn next(&mut self) -> TokenType {
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
            Some(ch) if ch == '<' && self.input[self.position + 1] == '<' => {
                (0..2).for_each(|_| self.read());
                let tag = self.read_identifier();
                let hear_document = self.read_hear_document(&tag);
                (0..tag.len()).for_each(|_| self.read());
                TokenType::HearDoc(hear_document)
            }
            Some(ch) if ch == '"' => {
                self.read();
                let value = self.read_value();
                self.read();
                TokenType::Literal(value)
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
    fn read_value(&mut self) -> String {
        let position = self.position;
        while self.examining_char.as_ref().map_or(false, |ch| ch != &'"') {
            self.read();
        }
        self.input[position..self.position]
            .iter()
            .collect::<String>()
            .as_str()
            .to_string()
    }
    fn read_hear_document(&mut self, tag: &str) -> String {
        let position = self.position;
        let pos = self.input[self.position..]
            .iter()
            .collect::<String>()
            .find(tag)
            .unwrap_or_else(|| panic!("hear document is not closed"));
        (0..pos).for_each(|_| self.read());
        self.input[position..(position + pos)]
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

struct Parser<'a> {
    lexer: &'a mut Lexer,
    current_token: Option<Box<TokenType>>,
    peek_token: Option<Box<TokenType>>,
}

impl<'a> Parser<'a> {
    fn new(lexer: &'a mut Lexer) -> Self {
        let mut parser = Self {
            lexer,
            current_token: Default::default(),
            peek_token: Default::default(),
        };
        parser.next_token();
        parser.next_token();
        parser
    }
    fn next_token(&mut self) {
        self.current_token = self.peek_token.take();
        self.peek_token = Some(Box::new(self.lexer.next()));
    }
    fn parse_resources(&mut self) -> Vec<Resource> {
        let mut resources: Vec<Resource> = vec![];
        while self
            .current_token
            .as_ref()
            .map_or(false, |token| token.as_ref() != &TokenType::EOF)
        {
            resources.push(self.parse_resource())
        }
        resources
    }
    fn parse_resource(&mut self) -> Resource {
        let resource_reserved_token = self
            .current_token
            .take()
            .unwrap_or_else(|| panic!("token is none"));
        let resource_reserved_literal = match resource_reserved_token.as_ref() {
            &TokenType::Literal(ref literal) => literal,
            _ => panic!("token is invalid. expect TokenType::Literal"),
        };
        if resource_reserved_literal != "resource" {
            panic!("token is invalid. expect resource");
        }
        self.next_token();

        let resource_name_token = self
            .current_token
            .take()
            .unwrap_or_else(|| panic!("token is none"));
        let resource_name_literal = match resource_name_token.as_ref() {
            &TokenType::Literal(ref literal) => literal,
            _ => panic!("token is invalid. expect TokenType::Literal"),
        };
        let resource_type = ResourceType::from_str(resource_name_literal);
        self.next_token();

        let event_name_token = self
            .current_token
            .take()
            .unwrap_or_else(|| panic!("token is none"));
        let event_name_literal = match event_name_token.as_ref() {
            &TokenType::Literal(ref literal) => literal,
            _ => panic!("token is invalid. expect TokenType::Literal"),
        };
        self.next_token();

        let lbrace_token = self
            .current_token
            .take()
            .unwrap_or_else(|| panic!("token is none"));
        if &TokenType::LBrace != lbrace_token.as_ref() {
            panic!("token is invalid. expect lbrace");
        }
        self.next_token();

        let attributes = self.parse_attributes();

        let rbrace_token = self
            .current_token
            .take()
            .unwrap_or_else(|| panic!("token is none"));
        if &TokenType::RBrace != rbrace_token.as_ref() {
            panic!("token is invalid. expect rbrace");
        }
        self.next_token();

        Resource::new(resource_type, event_name_literal.to_owned(), attributes)
    }
    fn parse_attributes(&mut self) -> HashMap<String, String> {
        let mut attributes = HashMap::<String, String>::new();
        while self
            .current_token
            .as_ref()
            .map_or(false, |token| token.as_ref() != &TokenType::RBrace)
        {
            let (key, value) = self.parse_attribute();
            attributes.insert(key, value);
        }
        attributes
    }
    fn parse_attribute(&mut self) -> (String, String) {
        let key_token = self
            .current_token
            .take()
            .unwrap_or_else(|| panic!("token is none"));
        let key_literal = match key_token.as_ref() {
            &TokenType::Literal(ref literal) => literal,
            _ => panic!("token is invalid. expect TokenType::Literal"),
        };
        self.next_token();

        if self
            .current_token
            .as_ref()
            .map_or(false, |token| token.as_ref() != &TokenType::Equal)
        {
            panic!("token is invalid. expect TokenType::Equal");
        }
        self.next_token();

        let value_token = self
            .current_token
            .take()
            .unwrap_or_else(|| panic!("token is none"));
        let value_literal = match value_token.as_ref() {
            &TokenType::Literal(ref literal) => literal.clone(),
            &TokenType::HearDoc(ref hear_doc) => hear_doc.clone(),
            &TokenType::LBrace => {
                self.next_token();
                let attr = self.parse_attributes();
                if Some(&Box::new(TokenType::RBrace)) != self.current_token.as_ref() {
                    panic!("token is invalid. expect rbrace");
                }
                format!("{:?}", attr)
            }
            _ => panic!("token is invalid. expect TokenType::Literal or TokenType::HearDoc"),
        };
        self.next_token();

        (key_literal.clone(), value_literal.clone())
    }
}
