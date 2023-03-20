use crate::core::expression::Expr;

pub struct LiteralExpr<T> {
  value: T
}

impl<T> LiteralExpr<T> {
  pub fn new(value: T) -> LiteralExpr<T> {
    LiteralExpr { value: value }
  }
}

#[derive(Debug)]
pub enum Literal {
  String(String),
  Integer(i64),
  Float(f64),
  Boolean(bool),
  Nil
}

impl Into<Expr> for Literal {
  fn into(self) -> Expr {
      Expr::Literal(self)
  }
}

impl Literal {
  pub fn new(value: &str) -> Literal {
    match value.parse::<i64>() {
      Ok(integer) => Literal::Integer(integer),
      Err(_) => match value.parse::<f64>() {
        Ok(float) => Literal::Float(float),
        Err(_) => match value {
          "true" => Literal::Boolean(true),
          "false" => Literal::Boolean(false),
          "nil" => Literal::Nil,
          _ => Literal::String(value.to_string())
        }
      }
    }
  }
}