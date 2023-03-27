use crate::{token::tokens::Token, core::expression::Expr};

#[derive(Clone, Debug)]
pub struct LetStatement {
    pub name: Token,
    pub initializer: Expr,
}

impl LetStatement {
    pub fn new(name: Token, initializer: Expr) -> LetStatement {
        LetStatement {
            name,
            initializer,
        }
    }
}
