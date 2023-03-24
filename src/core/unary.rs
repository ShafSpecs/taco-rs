use crate::core::expression::Expr;
use crate::into_expr;
use crate::token::tokens::Token;

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    operator: Token,
    right: Expr,
}

impl UnaryExpr {
    pub fn new(operator: Token, right: Expr) -> UnaryExpr {
        UnaryExpr { operator, right }
    }

    pub fn get_operator(&self) -> &Token {
        &self.operator
    }

    pub fn get_right(&self) -> &Expr {
        &self.right
    }
}

into_expr!(UnaryExpr);
