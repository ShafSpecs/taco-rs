use crate::core::grouping::GroupingExpr;
use crate::core::literal::Literal;
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

    fn expression(&mut self) -> Expr {
        return self.equality();
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = BinaryExpr::new(expr, operator, right).into();
        }

        return expr;
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = BinaryExpr::new(expr, operator, right).into();
        }

        return expr;
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = BinaryExpr::new(expr, operator, right).into();
        }

        return expr;
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = BinaryExpr::new(expr, operator, right).into();
        }

        return expr;
    }

    fn unary(&mut self) -> Expr {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return UnaryExpr::new(operator, right).into();
        }

        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        if self.match_tokens(&[TokenType::False]) {
            return Expr::Literal(Literal::Boolean(false));
        }

        if self.match_tokens(&[TokenType::True]) {
            return Expr::Literal(Literal::Boolean(true));
        }

        if self.match_tokens(&[TokenType::Nil]) {
            return Expr::Literal(Literal::Nil);
        }

        if self.match_tokens(&[TokenType::Integer, TokenType::Float, TokenType::String]) {
            return Expr::Literal(Literal::new(self.previous().lexeme.as_str()));
        }

        if self.match_tokens(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expected ')' after expression.");
            return Expr::GroupingExpr(Box::new(GroupingExpr::new(expr)));
        }

        panic!("Expect expression.");
    }

    fn consume(&mut self, token: TokenType, message: &str) -> Token {
        if self.check(token) {
            return self.advance();
        }

        panic!("{}", message);
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
}
