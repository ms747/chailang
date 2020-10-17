use crate::token::Token;

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}
