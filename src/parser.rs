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
    Colon,
    Equal,
    HearDocTag,
    Literal(String),
    Illegal(char),
    EOF,
}

#[derive(Debug, Default)]
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
        while self.examining_char.map_or(false, char::is_whitespace) {
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
            Some(ch) if ch == ':' => {
                self.read();
                TokenType::Colon
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
                let value = self.read_string();
                match self.examining_char {
                    Some(c) if c == '"' => (),
                    Some(c) => panic!(
                        "invalid char. expected: '\"', actual: {}, value: {}",
                        c, value
                    ),
                    None => panic!("invalid char. expected: '\"', actual: None"),
                }
                self.read();
                TokenType::Literal(value)
            }
            Some(ch) if Self::is_value(&ch) => {
                let value = self.read_ident();
                TokenType::Literal(value)
            }
            Some(ch) => TokenType::Illegal(ch),
            None => TokenType::EOF,
        }
    }

    fn read_ident(&mut self) -> String {
        let position = self.position;

        while Self::is_value(&self.examining_char.unwrap()) {
            self.read();
        }

        self.input[position..self.position]
            .iter()
            .collect::<String>()
            .as_str()
            .into()
    }

    fn read_string(&mut self) -> String {
        let position = self.position;

        while self.examining_char.map_or(false, |c| c != '"') {
            self.read();
        }

        self.input[position..self.position]
            .iter()
            .collect::<String>()
            .as_str()
            .into()
    }

    fn is_value(ch: &char) -> bool {
        ch.is_alphanumeric() || ['.', ':', '-', '_', '*'].contains(ch)
    }
}

#[derive(Debug)]
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
        println!("{:?}", self.current_token);
    }

    fn parse_resources(&mut self) -> Vec<Resource> {
        let mut resources: Vec<Resource> = vec![];

        while self.current_token_is_not(TokenType::EOF) {
            resources.push(self.parse_resource())
        }

        resources
    }

    fn parse_resource(&mut self) -> Resource {
        let resource_reserved_token = self.current_token.take().unwrap();
        let resource_literal = match resource_reserved_token.as_ref() {
            &TokenType::Literal(ref value) => value,
            tt => self.invalid_token(TokenType::Literal("".into()), tt, line!(), column!()),
        };
        self.expect_value("resource", &resource_literal, line!(), column!());
        self.next_token();

        let resource_kind_token = self.current_token.take().unwrap();
        let resource_kind_literal = match resource_kind_token.as_ref() {
            &TokenType::Literal(ref value) => value,
            tt => self.invalid_token(TokenType::Literal("".into()), tt, line!(), column!()),
        };
        let resource_type = ResourceKind::from_str(resource_kind_literal);
        self.next_token();

        let resource_name_token = self.current_token.take().unwrap();
        let resource_name_literal = match resource_name_token.as_ref() {
            &TokenType::Literal(ref value) => value,
            tt => self.invalid_token(TokenType::Literal("".into()), tt, line!(), column!()),
        };
        self.next_token();

        let attributes = self.parse_attributes();

        Resource::new(resource_type, resource_name_literal.into(), attributes)
    }

    fn parse_attributes(&mut self) -> HashMap<String, ValueContainer> {
        let lbrace_token = self.current_token.take().unwrap();
        self.expect_token(TokenType::LBrace, &lbrace_token, line!(), column!());
        self.next_token();

        let mut attributes = HashMap::<String, ValueContainer>::new();

        while !self.current_token_is(TokenType::RBrace) {
            let (key, value) = self.parse_attribute();
            attributes.insert(key, value);
            self.next_token();
        }

        let rbrace_token = self.current_token.take().unwrap();
        self.expect_token(TokenType::RBrace, &rbrace_token, line!(), column!());
        self.next_token();

        attributes
    }

    fn parse_attribute(&mut self) -> (String, ValueContainer) {
        let ident_token = self.current_token.take().unwrap();
        let ident_literal = match ident_token.as_ref() {
            &TokenType::Literal(ref value) => value,
            tt => self.invalid_token(TokenType::Literal("".into()), tt, line!(), column!()),
        };
        self.next_token();

        let equal_token = self.current_token.take().unwrap();
        self.expect_token(TokenType::Equal, &equal_token, line!(), column!());
        let peek_is_hear_doc_symbol = self.peek_token_is(TokenType::HearDocTag);
        self.next_token();

        let value = if peek_is_hear_doc_symbol {
            self.parse_hear_doc()
        } else {
            self.parse_value()
        };
        (ident_literal.into(), value)
    }

    fn parse_value(&mut self) -> ValueContainer {
        match self.current_token.as_ref().unwrap().as_ref() {
            &TokenType::LBrace => self.parse_dictionary(),
            &TokenType::LBracket => self.parse_array(),
            _ => self.parse_atom(),
        }
    }

    fn parse_json_value(&mut self) -> ValueContainer {
        match self.current_token.as_ref().unwrap().as_ref() {
            &TokenType::LBrace => self.parse_json(),
            &TokenType::LBracket => self.parse_array(),
            _ => self.parse_atom(),
        }
    }

    fn parse_hear_doc(&mut self) -> ValueContainer {
        let hear_doc_symbol_token = self.current_token.take().unwrap();
        self.expect_token(
            TokenType::HearDocTag,
            &hear_doc_symbol_token,
            line!(),
            column!(),
        );
        self.next_token();

        let tag_token = self.current_token.take().unwrap();
        let start_tag = match tag_token.as_ref() {
            &TokenType::Literal(ref tag) => tag,
            tt => self.invalid_token(TokenType::Literal("".into()), tt, line!(), column!()),
        };
        if !start_tag.chars().all(char::is_uppercase) {
            panic!("tag is invalid. expect all uppercase");
        }
        self.next_token();

        let json = self.parse_json();
        self.next_token();

        let tag_token = self.current_token.take().unwrap();
        let end_tag = match tag_token.as_ref() {
            &TokenType::Literal(ref tag) => tag,
            tt => self.invalid_token(TokenType::Literal("".into()), tt, line!(), column!()),
        };
        if !end_tag.chars().all(char::is_uppercase) && start_tag == end_tag {
            panic!("tag is invalid. expect all uppercase");
        }

        json
    }

    fn parse_dictionary(&mut self) -> ValueContainer {
        let lbrace_token = self.current_token.take().unwrap();
        self.expect_token(TokenType::LBrace, &lbrace_token, line!(), column!());
        self.next_token();

        let mut dictionary = HashMap::<String, ValueContainer>::new();

        while !self.current_token_is(TokenType::RBrace) {
            let (key, value) = self.parse_key_value();
            dictionary.insert(key, value);
            self.next_token();
        }

        let rbrace_token = self.current_token.take().unwrap();
        self.expect_token(TokenType::RBrace, &rbrace_token, line!(), column!());

        ValueContainer::Dictionary(dictionary)
    }

    fn parse_key_value(&mut self) -> (String, ValueContainer) {
        let ident_token = self.current_token.take().unwrap();
        let ident_literal = match ident_token.as_ref() {
            &TokenType::Literal(ref value) if self.is_ident(value) => value,
            &TokenType::Literal(ref l) => self.invalid_value("<IDENT>", l, line!(), column!()),
            tt => self.invalid_token(TokenType::Literal("".into()), tt, line!(), column!()),
        };
        self.next_token();

        let equla_token = self.current_token.take().unwrap();
        self.expect_token(TokenType::Equal, &equla_token, line!(), column!());
        self.next_token();

        let value = self.parse_value();

        (ident_literal.into(), value)
    }

    fn parse_json(&mut self) -> ValueContainer {
        let lbrace_token = self.current_token.take().unwrap();
        self.expect_token(TokenType::LBrace, &lbrace_token, line!(), column!());
        self.next_token();

        let mut dictionary = HashMap::<String, ValueContainer>::new();

        while !self.current_token_is(TokenType::RBrace) {
            let (key, value) = self.parse_json_key_value();
            dictionary.insert(key, value);
            self.next_token();

            if self.current_token_is(TokenType::Comma) {
                self.next_token();
            }
        }

        let rbrace_token = self.current_token.take().unwrap();
        self.expect_token(TokenType::RBrace, &rbrace_token, line!(), column!());

        ValueContainer::Dictionary(dictionary)
    }

    fn parse_json_key_value(&mut self) -> (String, ValueContainer) {
        let ident_token = self.current_token.take().unwrap();
        let ident_literal = match ident_token.as_ref() {
            &TokenType::Literal(ref value) if self.is_ident(value) => value,
            &TokenType::Literal(ref l) => self.invalid_value("<IDENT>", l, line!(), column!()),
            tt => self.invalid_token(TokenType::Literal("".into()), tt, line!(), column!()),
        };
        self.next_token();

        let colon_token = self.current_token.take().unwrap();
        self.expect_token(TokenType::Colon, &colon_token, line!(), column!());
        self.next_token();

        let value = self.parse_json_value();

        (ident_literal.into(), value)
    }

    fn parse_array(&mut self) -> ValueContainer {
        let lbracket_token = self.current_token.take().unwrap();
        self.expect_token(TokenType::LBracket, &lbracket_token, line!(), column!());
        self.next_token();

        let mut vec: Vec<Box<ValueContainer>> = vec![];

        while !self.current_token_is(TokenType::RBracket) {
            let value_container = self.parse_json_value();
            vec.push(Box::new(value_container));
            self.next_token();

            if self.current_token_is(TokenType::Comma) {
                self.next_token();
            }
        }

        let rbracket_token = self.current_token.take().unwrap();
        self.expect_token(TokenType::RBracket, &rbracket_token, line!(), column!());

        ValueContainer::Array(vec)
    }

    fn parse_atom(&mut self) -> ValueContainer {
        let token = self.current_token.take().unwrap();
        let value = match token.as_ref() {
            &TokenType::Literal(ref value) => value,
            tt => self.invalid_token(TokenType::Literal("".into()), tt, line!(), column!()),
        };

        let atom = if let Ok(n) = value.parse::<i32>() {
            ValueType::Number(n)
        } else if let Ok(b) = value.parse::<bool>() {
            ValueType::Bool(b)
        } else if value.is_empty() {
            ValueType::Str("".into())
        } else {
            ValueType::Str(value.into())
        };

        ValueContainer::Value(atom)
    }

    fn is_ident(&self, ident: &str) -> bool {
        let mut chars = ident.chars();
        chars.all(|c| c.is_alphanumeric() || ['.', ':', '-', '_', '*'].contains(&c))
    }

    fn current_token_is(&self, expect: TokenType) -> bool {
        self.current_token
            .as_ref()
            .map_or(false, |token| token.as_ref() == &expect)
    }

    fn current_token_is_not(&self, expect: TokenType) -> bool {
        !self.current_token_is(expect)
    }

    fn peek_token_is(&self, expect: TokenType) -> bool {
        self.peek_token
            .as_ref()
            .map_or(false, |token| token.as_ref() == &expect)
    }
}

impl<'a> Parser<'a> {
    fn expect_token(&self, expect: TokenType, actual: &TokenType, line: u32, column: u32) {
        if &expect != actual {
            self.invalid_token(expect, actual, line, column)
        }
    }

    fn invalid_token(&self, expect: TokenType, actual: &TokenType, line: u32, column: u32) -> ! {
        // dbg!(self);
        panic!(
            "[{}:{}]: token is invalid. expect: {:?}, actual: {:?}",
            line, column, expect, actual
        );
    }

    fn expect_value<T: std::fmt::Debug + ?Sized + PartialEq>(
        &self,
        expect: &T,
        actual: &T,
        line: u32,
        column: u32,
    ) {
        if expect != actual {
            self.invalid_value(expect, actual, line, column);
        }
    }

    fn invalid_value<T: std::fmt::Debug + ?Sized>(
        &self,
        expect: &T,
        actual: &T,
        line: u32,
        column: u32,
    ) -> ! {
        // dbg!(self);
        panic!(
            "[{}:{}]: value is invalid. expect: {:?}, actual: {:?}",
            line, column, expect, actual
        );
    }
}
