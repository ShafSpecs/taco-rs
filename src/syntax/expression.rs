use crate::core::expression::Expr;

pub struct ExpressionStatement {
    pub expression: Expr
}

impl ExpressionStatement {
    pub fn new(expression: Expr) -> ExpressionStatement {
        ExpressionStatement {
            expression
        }
    }
}
