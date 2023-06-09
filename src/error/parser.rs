use std::fmt::Display;

use crate::token::tokens::{Token, TokenType};
use crate::error::handling::report;

pub fn throw_error(token: Token, message: &str) -> ParserError {
  if token.token_type == TokenType::Eof {
    report(token.line as u8, " at end", message);
  } else {
    report(token.line as u8, &format!(" at '{}'", token.lexeme), message);
  }

  return ParserError::new(vec![token], message);
}

#[derive(Debug, Clone)]
pub struct ParserError {
  pub token: Vec<Token>,
  pub message: String,
}

impl Display for ParserError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "[Line {}] {}",
      self.token[0].get_line(),
      self.message
    )
  }
}

impl ParserError {
  pub fn new(token: Vec<Token>, message: &str) -> ParserError {
    ParserError {
      token,
      message: message.to_string(),
    }
  }

  pub fn get_token(&self) -> &Vec<Token> {
    &self.token
  }
}