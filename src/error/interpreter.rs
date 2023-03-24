use std::fmt::{Display, Formatter};

use crate::token::tokens::Token;

pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[Line {}] Error: {}",
            self.token.get_line(),
            self.message
        )
    }
}

impl RuntimeError {
    pub fn new(token: Token, message: &str) -> RuntimeError {
        RuntimeError {
            token,
            message: message.to_string(),
        }
    }

    pub fn get_token(&self) -> &Token {
        &self.token
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }
}