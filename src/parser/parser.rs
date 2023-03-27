use crate::core::grouping::GroupingExpr;
use crate::core::literal::Literal;
use crate::error::parser::{throw_error, ParserError};
use crate::syntax::expression::ExpressionStatement;
use crate::syntax::print::PrintStatement;
use crate::syntax::r#let::LetStatement;
use crate::syntax::statement::Statement;
use crate::{
    core::{binary::BinaryExpr, expression::Expr, unary::UnaryExpr},
    token::tokens::{Token, TokenType},
};

#[macro_export]
macro_rules! into_expr {
    ($id:ident) => {
        impl Into<Expr> for $id {
            fn into(self) -> Expr {
                Expr::$id(Box::new(self))
            }
        }
    };
}

pub struct Parser {
    pub tokens: Vec<Token>,
    current: u32,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, ParserError> {
        let mut statements = Vec::<Statement>::new();

        while !self.is_at_end() {
            let dec = match self.declaration() {
                Ok(dec) => dec,
                Err(err) => {
                    self.synchronize();
                    return Err(ParserError::new(err.token, err.message.as_str()));
                }
            };

            statements.push(dec);
        }

        return Ok(statements);
    }

    fn declaration(&mut self) -> Result<Statement, ParserError> {
        if self.match_tokens(&[TokenType::Let]) {
            return self.let_declaration();
        }

        if self.match_tokens(&[TokenType::Print]) {
            return self.print_statement();
        }

        return self.expr_statement();
    }

    fn let_declaration(&mut self) -> Result<Statement, ParserError> {
        let name = match self.consume(TokenType::Identifier, "Expect variable name.") {
            Ok(name) => name,
            Err(err) => return Err(err),
        };

        let mut initializer = Expr::Literal(Literal::Nil);

        if self.match_tokens(&[TokenType::Equal]) {
            let value = match self.expression() {
                Ok(expr) => expr,
                Err(err) => return Err(err),
            };

            initializer = value.clone().into();
        }

        if self.match_tokens(&[TokenType::Semicolon]) {
            return Ok(Statement::LetStatement(LetStatement::new(name, initializer)));
        }

        return Err(throw_error(self.previous(), "Expect ';' after variable declaration."));
    }

    fn print_statement(&mut self) -> Result<Statement, ParserError> {
        let value = match self.expression() {
            Ok(expr) => expr,
            Err(err) => return Err(err),
        };

        if self.match_tokens(&[TokenType::Semicolon]) {
            return Ok(Statement::PrintStatement(PrintStatement::new(value)));
        }

        return Err(throw_error(self.peek(), "Expect ';' after value."));
    }

    fn expr_statement(&mut self) -> Result<Statement, ParserError> {
        let expr = match self.expression() {
            Ok(expr) => expr,
            Err(err) => return Err(err),
        };

        if self.match_tokens(&[TokenType::Semicolon]) {
            return Ok(Statement::ExpressionStatement(ExpressionStatement::new(
                expr,
            )));
        }

        return Err(throw_error(self.peek(), "Expect ';' after expression."));
    }

    fn expression(&mut self) -> Result<Expr, ParserError> {
        return self.equality();
    }

    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.comparison();

        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = match self.comparison() {
                Ok(expr) => expr,
                Err(err) => return Err(err),
            };
            expr = match expr {
                Ok(expr) => Ok(BinaryExpr::new(expr, operator, right).into()),
                Err(err) => return Err(err),
            }
        }

        return expr;
    }

    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.term();

        while self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = match self.term() {
                Ok(expr) => expr,
                Err(err) => return Err(err),
            };
            expr = match expr {
                Ok(expr) => Ok(BinaryExpr::new(expr, operator, right).into()),
                Err(err) => return Err(err),
            };
        }

        return expr;
    }

    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.factor();

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = match self.factor() {
                Ok(expr) => expr,
                Err(err) => return Err(err),
            };
            expr = match expr {
                Ok(expr) => Ok(BinaryExpr::new(expr, operator, right).into()),
                Err(err) => return Err(err),
            };
        }

        return expr;
    }

    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.unary();

        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = match self.unary() {
                Ok(expr) => expr,
                Err(err) => return Err(err),
            };

            expr = match expr {
                Ok(expr) => Ok(BinaryExpr::new(expr, operator, right).into()),
                Err(err) => return Err(err),
            };
        }

        return expr;
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();

            let right = match self.unary() {
                Ok(expr) => expr,
                Err(err) => return Err(err),
            };
            return Ok(UnaryExpr::new(operator, right).into());
        }

        return self.primary();
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {
        if self.match_tokens(&[TokenType::False]) {
            return Ok(Expr::Literal(Literal::Boolean(false)));
        }

        if self.match_tokens(&[TokenType::True]) {
            return Ok(Expr::Literal(Literal::Boolean(true)));
        }

        if self.match_tokens(&[TokenType::Nil]) {
            return Ok(Expr::Literal(Literal::Nil));
        }

        if self.match_tokens(&[TokenType::Integer, TokenType::Float, TokenType::String]) {
            return Ok(Expr::Literal(Literal::new(self.previous().lexeme.as_str())));
        }

        if self.match_tokens(&[TokenType::Identifier]) {
            return Ok(Expr::VarDeclaration(self.previous()));
        }

        if self.match_tokens(&[TokenType::LeftParen]) {
            let expr = match self.expression() {
                Ok(expr) => expr,
                Err(err) => return Err(err),
            };
            self.consume(TokenType::RightParen, "Expected ')' after expression.");
            return Ok(Expr::GroupingExpr(Box::new(GroupingExpr::new(expr))));
        }

        Err(throw_error(self.peek(), "Expected expression"))
    }

    fn consume(&mut self, token: TokenType, message: &str) -> Result<Token, ParserError> {
        match self.check(token) {
            true => Ok(self.advance()),
            false => Err(throw_error(self.peek(), message)),
        }
    }

    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(*token_type) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn check(&mut self, token: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        return self.peek().token_type == token;
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1
        };
        return self.previous();
    }

    fn is_at_end(&mut self) -> bool {
        return self.peek().token_type == TokenType::Eof;
    }

    fn peek(&mut self) -> Token {
        return self.tokens.get(self.current as usize).unwrap().clone();
    }

    fn previous(&mut self) -> Token {
        let current = self.current.clone() as usize;
        return self.tokens.get(current - 1).unwrap().clone();
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Func
                | TokenType::Let
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => self.advance(),
            };
        }
    }
}
