use crate::syntax::expression::ExpressionStatement;
use crate::syntax::print::PrintStatement;

use super::r#let::LetStatement;

#[derive(Clone, Debug)]
pub enum Statement {
    ExpressionStatement(ExpressionStatement),
    PrintStatement(PrintStatement),
    LetStatement(LetStatement),
    // BlockStatement(BlockStatement),
    // IfStatement(IfStatement),
    // WhileStatement(WhileStatement),
    // FunctionStatement(FunctionStatement),
    // ReturnStatement(ReturnStatement),
    // ClassStatement(ClassStatement)
}
