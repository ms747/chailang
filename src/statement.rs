use crate::expression::Expression;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let(String, Box<Expression>),
    Return(Box<Expression>),
    ExpressionStatement(Box<Expression>),
    BlockStatement(Vec<Statement>),
    Reassignment(String, Box<Expression>),
    Program(Vec<Statement>),
}
