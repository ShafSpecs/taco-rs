use crate::core::expression::Expr;
use crate::into_expr;
use crate::token::tokens::Token;

#[derive(Debug)]
pub struct UnaryExpr {
    operator: Token,
    right: Expr,
}

impl UnaryExpr {
    pub fn new(operator: Token, right: Expr) -> UnaryExpr {
        UnaryExpr { operator, right }
    }
}

into_expr!(UnaryExpr);
