use std::collections::HashMap;

use crate::error::handling::error;
use crate::token::tokens::{Token, TokenType};

pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    pub keywords: HashMap<String, TokenType>,
    start: u32,
    current: u32,
    line: u8,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords: Scanner::create_keywords()
        }
    }

    fn create_keywords() -> HashMap<String, TokenType> {
      let mut map = HashMap::new();
      map.insert(String::from("and"), TokenType::And);
      map.insert(String::from("class"), TokenType::Class);
      map.insert(String::from("else"), TokenType::Else);
      map.insert(String::from("false"), TokenType::False);
      map.insert(String::from("for"), TokenType::For);
      map.insert(String::from("taco"), TokenType::Func);
      map.insert(String::from("if"), TokenType::If);
      map.insert(String::from("nil"), TokenType::Nil);
      map.insert(String::from("or"), TokenType::Or);
      map.insert(String::from("print"), TokenType::Print);
      map.insert(String::from("return"), TokenType::Return);
      map.insert(String::from("super"), TokenType::Super);
      map.insert(String::from("this"), TokenType::This);
      map.insert(String::from("true"), TokenType::True);
      map.insert(String::from("let"), TokenType::Let);
      map.insert(String::from("while"), TokenType::While);

      return map;
    }

    fn get_keywords(&mut self) -> HashMap<String, TokenType> {
      return self.keywords.clone();
    }

    fn is_at_end(&mut self) -> bool {
        return self.current >= self.source.len() as u32;
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        return self
            .source
            .chars()
            .nth((self.current - 1) as usize)
            .unwrap();
    }

    fn add_token(&mut self, type_: TokenType) -> () {
        self.add_token_with_literal(type_, String::new());
    }

    fn add_token_with_literal(&mut self, type_: TokenType, literal: String) -> () {
        let text = self
            .source
            .chars()
            .skip(self.start as usize)
            .take((self.current - self.start) as usize)
            .collect::<String>();
        self.tokens
            .push(Token::new(type_, text, literal, self.line.into()));
    }

    fn match_next(&mut self, token: char) -> bool {
        return self.source.chars().nth(self.current as usize).unwrap() == token;
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        return self.source.chars().nth(self.current as usize).unwrap();
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() as u32 {
            return '\0';
        }

        return self.source.chars().nth((self.current + 1) as usize).unwrap();
    }

    fn handle_string(&mut self) -> () {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                error(self.line, "Unterminated string");
                return;
            }
            self.advance();
        }

        if self.is_at_end() {
            error(self.line, "Unterminated string");
            return;
        }

        self.advance();

        let value = self
            .source
            .chars()
            .skip((self.start + 1) as usize)
            .take((self.current - self.start - 2) as usize)
            .collect::<String>();
        self.add_token_with_literal(TokenType::String, value);
    }

    fn handle_multiline_string(&mut self) -> () {
        while self.peek() != '`' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            error(self.line, "Unterminated string");
            return;
        }

        self.advance();

        let value = self
            .source
            .chars()
            .skip((self.start + 1) as usize)
            .take((self.current - self.start - 2) as usize)
            .collect::<String>();
        self.add_token_with_literal(TokenType::String, value);
    }

    fn handle_multiline_comment(&mut self) -> () {
        while self.peek() != '*' && self.peek_next() != '/' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        self.advance();
        self.advance();
    }
    
    fn is_a_digit(character: char) -> bool {
        return character.is_digit(10);
    }

    fn handle_number(&mut self) -> () {
        let mut is_float = false;

        while Scanner::is_a_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && Scanner::is_a_digit(self.peek_next()) {
            is_float = true;
            self.advance();

            while Scanner::is_a_digit(self.peek()) {
                self.advance();
            }
        }

        let value = self
            .source
            .chars()
            .skip(self.start as usize)
            .take((self.current - self.start) as usize)
            .collect::<String>();

        if is_float {
            self.add_token_with_literal(TokenType::Float, value);
            return;
        }

        self.add_token_with_literal(TokenType::Integer, value);
    }

    fn is_alpha(character: char) -> bool {
        return character.is_alphabetic() || character == '_';
    }

    fn is_alphanumeric(character: char) -> bool {
      return Scanner::is_a_digit(character) || Scanner::is_alpha(character);
    }

    fn handle_identifier(&mut self) -> () {
      while Scanner::is_alphanumeric(self.peek()) {
        self.advance();
      }

      let text = self
        .source
        .chars()
        .skip(self.start as usize)
        .take((self.current - self.start) as usize)
        .collect::<String>();

      let type_ = self.get_keywords().get(&text).unwrap_or(&TokenType::Identifier).clone();
      self.add_token(type_);
    }

    fn scan_token(&mut self) -> () {
        let c = self.advance();

        match c {
            '(' => self.add_token_with_literal(TokenType::LeftParen, String::from("(")),
            ')' => self.add_token_with_literal(TokenType::RightParen, String::from(")")),
            '{' => self.add_token_with_literal(TokenType::LeftBrace, String::from("{")),
            '}' => self.add_token_with_literal(TokenType::RightBrace, String::from("}")),
            ',' => self.add_token_with_literal(TokenType::Comma, String::from(",")),
            '.' => self.add_token_with_literal(TokenType::Dot, String::from(".")),
            '-' => self.add_token_with_literal(TokenType::Minus, String::from("-")),
            '+' => self.add_token_with_literal(TokenType::Plus, String::from("+")),
            ';' => self.add_token_with_literal(TokenType::Semicolon, String::from(";")),
            '*' => self.add_token_with_literal(TokenType::Star, String::from("*")),
            ////* Two character tokens
            '!' => {
                if self.match_next('=') {
                    self.add_token_with_literal(TokenType::BangEqual, String::from("!="));
                    self.advance();
                } else {
                    self.add_token_with_literal(TokenType::Bang, String::from("!"));
                }
            }
            '=' => {
                if self.match_next('=') {
                    self.add_token_with_literal(TokenType::EqualEqual, String::from("=="));
                    self.advance();
                } else {
                    self.add_token_with_literal(TokenType::Equal, String::from("="));
                }
            }
            '<' => {
                if self.match_next('=') {
                    self.add_token_with_literal(TokenType::LessEqual, String::from("<="));
                    self.advance();
                } else {
                    self.add_token_with_literal(TokenType::Less, String::from("<"));
                }
            }
            '>' => {
                if self.match_next('=') {
                    self.add_token_with_literal(TokenType::GreaterEqual, String::from(">="));
                    self.advance();
                } else {
                    self.add_token_with_literal(TokenType::Greater, String::from(">"));
                }
            }
            // End of two character tokens *//
            '#' => {
                while !self.is_at_end() && self.peek() != '\n' {
                    self.advance();
                }
            }
            '/' => {
                if self.match_next('*') {
                    self.handle_multiline_comment();
                } else {
                    self.add_token_with_literal(TokenType::Slash, String::from("/"));
                }
            }
            ' ' | '\r' | '\t' => {
                // Ignore whitespace
            }
            '\n' => {
                self.line += 1;
            }
            '"' => {
                self.handle_string();
            }
            '`' => {
                self.handle_multiline_string();
            }
            _ => {
                if Scanner::is_a_digit(c) {
                    self.handle_number();
                } else if Scanner::is_alpha(c) {
                    self.handle_identifier();
                } else {
                    error(self.line, "Unexpected character found");
                }
            }
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            String::from(""),
            String::from(""),
            self.line.into(),
        ));

        return self.tokens.clone();
    }
}
