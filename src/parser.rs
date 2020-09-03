use crate::model::{Resource, ResourceKind, ValueContainer, ValueType};
use std::collections::HashMap;

pub(crate) fn parse(content: String) -> Vec<Resource> {
    if content.is_empty() {
        return vec![];
    }

    let mut lexer = Lexer::new(content);
    let mut parser = Parser::new(&mut lexer);
    parser.parse_resources()
}

#[derive(Clone, PartialEq, Debug)]
enum TokenType {
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Quote,
    Equal,
    HearDocTag,
    Literal(String),
    Illegal(char),
    EOF,
}

#[derive(Default)]
struct Lexer {
    input: Vec<char>,
    position: usize,
    peek_position: usize,
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

    fn read(&mut self) {
        self.examining_char = self.input.get(self.peek_position).cloned();
        self.position = self.peek_position;
        self.peek_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while char::is_whitespace(
            self.examining_char
                .unwrap_or_else(|| panic!("examining char is none")),
        ) {
            self.read();
        }
    }

    fn next(&mut self) -> TokenType {
        self.skip_whitespace();
        match self.examining_char {
            Some(ch) if ch == '{' => {
                self.read();
                TokenType::LBrace
            }
            Some(ch) if ch == '}' => {
                self.read();
                TokenType::RBrace
            }
            Some(ch) if ch == '[' => {
                self.read();
                TokenType::LBracket
            }
            Some(ch) if ch == ']' => {
                self.read();
                TokenType::RBracket
            }
            Some(ch) if ch == ',' => {
                self.read();
                TokenType::Comma
            }
            Some(ch) if ch == '=' => {
                self.read();
                TokenType::Equal
            }
            Some(ch) if ch == '<' && self.input[self.peek_position] == '<' => {
                (0..2).for_each(|_| self.read());
                TokenType::HearDocTag
            }
            Some(ch) if ch == '"' => {
                self.read();
                TokenType::Quote
            }
            Some(ch) if Self::is_value(&ch) => {
                let value = self.read_value();
                TokenType::Literal(value)
            }
            Some(ch) => TokenType::Illegal(ch),
            None => TokenType::EOF,
        }
    }

    fn read_value(&mut self) -> String {
        let position = self.position;

        while Self::is_value(
            &self
                .examining_char
                .unwrap_or_else(|| panic!("examining char is none")),
        ) {
            self.read();
        }

        self.input[position..self.position]
            .iter()
            .collect::<String>()
            .as_str()
            .to_string()
    }

    fn is_value(ch: &char) -> bool {
        ch.is_alphanumeric() || ['.', ':', '-', '_'].contains(ch)
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
        let resource_literal = match resource_reserved_token.as_ref() {
            &TokenType::Literal(ref value) => value,
            _ => panic!("token is invalid. expect TokenType::Value"),
        };
        if resource_literal != "resource" {
            panic!("token is invalid. expect resource");
        }
        self.next_token();

        let quote_token = self
            .current_token
            .take()
            .unwrap_or_else(|| panic!("token is none"));
        if quote_token.as_ref() == &TokenType::Quote {
            panic!("token is invalid. expect TokenType::Quote")
        }
        self.next_token();

        let resource_kind_token = self
            .current_token
            .take()
            .unwrap_or_else(|| panic!("token is none"));
        let resource_kind_literal = match resource_kind_token.as_ref() {
            &TokenType::Literal(ref value) => value,
            _ => panic!("token is invalid. expect TokenType::Value"),
        };
        let resource_type = ResourceKind::from_str(resource_kind_literal);
        self.next_token();

        let quote_token = self
            .current_token
            .take()
            .unwrap_or_else(|| panic!("token is none"));
        if quote_token.as_ref() == &TokenType::Quote {
            panic!("token is invalid. expect TokenType::Quote")
        }
        self.next_token();

        let quote_token = self
            .current_token
            .take()
            .unwrap_or_else(|| panic!("token is none"));
        if quote_token.as_ref() == &TokenType::Quote {
            panic!("token is invalid. expect TokenType::Quote")
        }
        self.next_token();

        let resource_name_token = self
            .current_token
            .take()
            .unwrap_or_else(|| panic!("token is none"));
        let resource_name_literal = match resource_name_token.as_ref() {
            &TokenType::Literal(ref value) => value,
            _ => panic!("token is invalid. expect TokenType::Value"),
        };
        self.next_token();

        let quote_token = self
            .current_token
            .take()
            .unwrap_or_else(|| panic!("token is none"));
        if quote_token.as_ref() == &TokenType::Quote {
            panic!("token is invalid. expect TokenType::Quote")
        }
        self.next_token();

        let lbrace_token = self
            .current_token
            .take()
            .unwrap_or_else(|| panic!("token is none"));
        if lbrace_token.as_ref() != &TokenType::LBrace {
            panic!("token is invalid. expect TokenType::LBrace");
        }
        self.next_token();

        let attributes = self.parse_attributes();

        let rbrace_token = self
            .current_token
            .take()
            .unwrap_or_else(|| panic!("token is none"));
        if rbrace_token.as_ref() != &TokenType::RBrace {
            panic!("token is invalid. expect TokenType::RBrace");
        }
        self.next_token();

        Resource::new(resource_type, resource_name_literal.to_owned(), attributes)
    }

    fn parse_attributes(&mut self) -> HashMap<String, ValueContainer> {
        let mut attributes = HashMap::<String, ValueContainer>::new();

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

    fn parse_attribute(&mut self) -> (String, ValueContainer) {
        let ident_token = self
            .current_token
            .take()
            .unwrap_or_else(|| panic!("token is none"));
        let ident_literal = match ident_token.as_ref() {
            &TokenType::Literal(ref value) => value,
            _ => panic!("token is invalid. expect TokenType::Value"),
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

        let value = if self
            .current_token
            .as_ref()
            .map_or(false, |token| token.as_ref() == &TokenType::HearDocTag)
        {
            self.next_token();
            self.parse_hear_doc()
        } else {
            self.next_token();
            ValueContainer::Value(self.parse_atom())
        };
        self.next_token();

        (ident_literal.to_owned(), value)
    }

    fn parse_hear_doc(&mut self) -> ValueContainer {
        let tag_token = self
            .current_token
            .take()
            .unwrap_or_else(|| panic!("token is none"));
        let start_tag = match tag_token.as_ref() {
            &TokenType::Literal(ref tag) => tag,
            _ => panic!("token is invalid. expect TokenType::Literal"),
        };
        if !start_tag.chars().all(char::is_uppercase) {
            panic!("tag is invalid. expect all uppercase");
        }
        self.next_token();

        let json = self.parse_json();

        let tag_token = self
            .current_token
            .take()
            .unwrap_or_else(|| panic!("token is none"));
        let end_tag = match tag_token.as_ref() {
            &TokenType::Literal(ref tag) => tag,
            _ => panic!("token is invalid. expect TokenType::Literal"),
        };
        if !end_tag.chars().all(char::is_uppercase) && start_tag == end_tag {
            panic!("tag is invalid. expect all uppercase");
        }
        self.next_token();

        json
    }

    fn parse_json(&mut self) -> ValueContainer {
        if self
            .current_token
            .as_ref()
            .map_or(false, |token| token.as_ref() != &TokenType::LBrace)
        {
            panic!("token is invalid. expect TokenType::LBrace");
        }
        self.next_token();

        let mut dictionary = HashMap::<String, ValueContainer>::new();

        while self
            .current_token
            .as_ref()
            .map_or(false, |token| token.as_ref() != &TokenType::RBrace)
        {
            let (key, value) = self.parse_dictionary();
            dictionary.insert(key, value);

            if self
                .current_token
                .as_ref()
                .map_or(false, |token| token.as_ref() != &TokenType::Comma)
            {
                panic!("Token is invalid. expect TokenType::Comma");
            }
            self.next_token();
        }

        ValueContainer::Dictionary(dictionary)
    }

    fn parse_dictionary(&mut self) -> (String, ValueContainer) {
        if self
            .current_token
            .as_ref()
            .map_or(false, |token| token.as_ref() != &TokenType::Quote)
        {
            panic!("token is invalid. expect TokenType::Quote");
        }
        self.next_token();

        let ident_token = self
            .current_token
            .take()
            .unwrap_or_else(|| panic!("token is none"));
        let ident_literal = match ident_token.as_ref() {
            &TokenType::Literal(ref value) if is_ident(value) => value,
            &TokenType::Literal(_) => panic!("token is invalid. expect ident literal"),
            _ => panic!("token is invalid. expect TokenType::Literal"),
        };
        self.next_token();

        if self
            .current_token
            .as_ref()
            .map_or(false, |token| token.as_ref() != &TokenType::Quote)
        {
            panic!("token is invalid. expect TokenType::Quote");
        }
        self.next_token();

        let value = self.parse_value();

        (ident_literal.clone(), value)
    }

    fn parse_value(&mut self) -> ValueContainer {
        match self
            .current_token
            .as_ref()
            .unwrap_or_else(|| panic!("token is none"))
            .as_ref()
        {
            &TokenType::LBrace => self.parse_json(),
            &TokenType::LBracket => self.parse_array(),
            _ => ValueContainer::Value(self.parse_atom()),
        }
    }

    fn parse_array(&mut self) -> ValueContainer {
        if self
            .current_token
            .as_ref()
            .map_or(false, |token| token.as_ref() != &TokenType::LBracket)
        {
            panic!("token is invalid. expect TokenType::LBrace");
        }
        self.next_token();

        let mut vec: Vec<Box<ValueContainer>> = vec![];

        while self
            .current_token
            .as_ref()
            .map_or(false, |token| token.as_ref() != &TokenType::RBracket)
        {
            let value_container = self.parse_value();
            vec.push(Box::new(value_container));

            if self
                .current_token
                .as_ref()
                .map_or(false, |token| token.as_ref() != &TokenType::Comma)
            {
                panic!("Token is invalid. expect TokenType::Comma");
            }
            self.next_token();
        }

        ValueContainer::Array(vec)
    }

    fn parse_atom(&mut self) -> ValueType {
        let token = self
            .current_token
            .take()
            .unwrap_or_else(|| panic!("token is none"));
        let value = match token.as_ref() {
            &TokenType::Literal(ref value) => value,
            _ => panic!("token is invalid. expect TokenType::Literal"),
        };
        match value {
            n if n.parse::<i32>().ok().is_some() => ValueType::Number(n.parse::<i32>().unwrap()),
            b if b.parse::<bool>().ok().is_some() => ValueType::Bool(b.parse::<bool>().unwrap()),
            s if is_ident(s) => ValueType::Str(s.clone()),
            _ => panic!("atom is invalid."),
        }
    }
}

fn is_ident(ident: &str) -> bool {
    let mut chars = ident.chars();
    let first = chars.next().unwrap_or_else(|| panic!("ident is empty"));
    if !first.is_alphabetic() {
        panic!("ident is invalid. expect alphabetic");
    }
    chars.all(|c| c.is_alphanumeric() || ['.', ':', '-', '_'].contains(&c))
}
