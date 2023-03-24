use crate::{core::expression::Expr, token::tokens::Token, into_expr};

#[derive(Debug, Clone)]
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

  pub fn get_left(&self) -> Expr {
    self.left.clone()
  }

  pub fn get_operator(&self) -> &Token {
    &self.operator
  }

  pub fn get_right(&self) -> Expr {
    self.right.clone()
  }
}