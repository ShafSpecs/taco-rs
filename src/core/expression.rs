use crate::core::binary::BinaryExpr;
use crate::core::grouping::GroupingExpr;
use crate::core::literal::Literal;
use crate::core::unary::UnaryExpr;

#[derive(Debug)]
pub enum Expr {
    BinaryExpr(Box<BinaryExpr>),
    GroupingExpr(Box<GroupingExpr>),
    UnaryExpr(Box<UnaryExpr>),
    // LiteralExpr(Box<LiteralExpr<Box<dyn Any>>>)
    Literal(Literal),
}
