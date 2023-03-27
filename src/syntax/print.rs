use crate::core::expression::Expr;

#[derive(Clone, Debug)]
pub struct PrintStatement {
    pub expression: Expr,
}

impl PrintStatement {
    pub fn new(expression: Expr) -> Self {
        Self { expression }
    }
}
