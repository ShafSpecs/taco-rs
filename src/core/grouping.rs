use crate::{core::expression::Expr, into_expr};

#[derive(Debug, Clone)]
pub struct GroupingExpr {
  pub expr: Expr
}

into_expr!(GroupingExpr);

impl GroupingExpr {
  pub fn new(expr: Expr) -> GroupingExpr {
    GroupingExpr { expr: expr }
  }
}