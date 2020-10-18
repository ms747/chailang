use crate::parser::Precedence;
use crate::tokeninfo::TokenInfo;

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum TokenType {
    // Keywords
    Function,
    While,
    Let,
    True,
    False,
    If,
    Else,
    Return,
    // Variables
    Ident,
    Int,
    String,
    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lessthan,
    Greaterthan,
    Equal,
    Notequal,
    // Separtors
    Comma,
    Semicolon,
    // Grouping
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Lbracket,
    Rbracket,
    // Illegal
    Illegal,
    // EOF
    Eof,
}

impl TokenType {
    pub fn precedence(token_type: &TokenType) -> Precedence {
        match token_type {
            TokenType::Equal => Precedence::Equals,
            TokenType::Notequal => Precedence::Equals,
            TokenType::Lessthan => Precedence::Lessgreater,
            TokenType::Greaterthan => Precedence::Lessgreater,
            TokenType::Plus => Precedence::Sum,
            TokenType::Minus => Precedence::Sum,
            TokenType::Asterisk => Precedence::Product,
            TokenType::Slash => Precedence::Product,
            TokenType::Lparen => Precedence::Call,
            TokenType::Lbracket => Precedence::Index,
            TokenType::Assign => Precedence::Assign,
            _ => Precedence::Lowest,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub token_info: TokenInfo,
}

impl Token {
    pub fn new(token_type: TokenType) -> Self {
        Token {
            token_type,
            token_info: TokenInfo::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn compare_tokens() {
        let assign_1 = Token::new(TokenType::Assign);
        let assign_2 = Token::new(TokenType::Assign);
        assert_eq!(assign_1, assign_2);
    }
}
