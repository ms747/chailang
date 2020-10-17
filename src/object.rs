use std::fmt::Display;

use crate::statement::Statement;

#[derive(Debug, Clone, PartialEq)]
pub struct Function(pub(crate) Vec<String>, pub(crate) Statement);

pub type BuildinFunction = fn(Vec<ChaiObject>) -> ChaiObject;

#[derive(Debug, PartialEq, Clone)]
pub enum ChaiObject {
    Integer(i32),
    Boolean(bool),
    String(String),
    Return(Box<ChaiObject>),
    Error(String),
    Function(Function),
    BuildinFunction(BuildinFunction),
    Array(Vec<ChaiObject>),
    Print(String),
    Null,
}

impl Display for ChaiObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChaiObject::Integer(_) => write!(f, "Integer"),
            ChaiObject::Boolean(_) => write!(f, "Boolean"),
            ChaiObject::String(_) => write!(f, "String"),
            ChaiObject::Null => write!(f, "Null"),
            _ => write!(f, "{{ Object }}"),
        }
    }
}
