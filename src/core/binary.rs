use crate::{core::expression::Expr, token::tokens::Token, into_expr};

#[derive(Debug)]
pub struct BinaryExpr {
  left: Expr,
  operator: Token,
  right: Expr
}

into_expr!(BinaryExpr);

impl BinaryExpr {
  pub fn new(left: Expr, operator: Token, right: Expr) -> BinaryExpr {
    BinaryExpr { left, operator, right }
  }
}