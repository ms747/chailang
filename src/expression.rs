use std::fmt::Display;

use crate::statement::Statement;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Null,
    Ident(String),
    String(String),
    Integer(i32),
    Prefix(Prefix, Box<Expression>),
    Infix(Box<Expression>, Operator, Box<Expression>),
    Boolean(bool),
    If(Box<Expression>, Statement, Option<Statement>),
    While(Box<Expression>, Statement),
    Function(Vec<String>, Statement),
    FunctionCall(Box<Expression>, Vec<Expression>),
    Array(Vec<Expression>),
    ArrayIndex(Box<Expression>, Box<Expression>),
    ReAssign(Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Prefix {
    Minus,
    Bang,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Greaterthan,
    Lessthan,
    Equals,
    Notequals,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut operator_string = String::new();
        match self {
            Operator::Plus => operator_string.push('+'),
            Operator::Minus => operator_string.push('-'),
            Operator::Multiply => operator_string.push('x'),
            Operator::Divide => operator_string.push('x'),
            Operator::Greaterthan => operator_string.push('>'),
            Operator::Lessthan => operator_string.push('<'),
            Operator::Equals => operator_string.push_str("=="),
            Operator::Notequals => operator_string.push_str("!="),
        };
        write!(f, "{}", operator_string)
    }
}
