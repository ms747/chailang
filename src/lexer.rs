use crate::token::Token;
use crate::token::TokenType;
use crate::tokeninfo::TokenInfo;

#[derive(Debug, Clone, PartialEq)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    line: usize,
    ch: u8,
    col: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            line: 1,
            col: 0,
            ch: 0,
        };
        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            return self.input.as_bytes()[self.read_position];
        }
    }

    fn create_token(&self, token_type: TokenType, start: usize, end: usize) -> Token {
        Token {
            token_type,
            token_info: TokenInfo {
                litertal: self.input[start..end].to_string(),
                line: self.line,
                col: (start + 1 - self.col),
            },
        }
    }

    fn create_eof(&self) -> Token {
        Token {
            token_type: TokenType::Eof,
            token_info: TokenInfo::default(),
        }
    }

    fn skip_whitespace(&mut self) {
        while (self.ch as char).is_whitespace() {
            if self.ch == b'\n' {
                self.col = self.read_position;
                self.line += 1;
            }
            self.read_char();
        }
    }

    fn is_letter(ch: u8) -> bool {
        ch >= b'A' && ch <= b'Z' || ch >= b'a' && ch <= b'z' || ch == b'_'
    }

    fn is_digit(ch: u8) -> bool {
        ch >= b'0' && ch <= b'9'
    }

    fn read_identifier(&mut self) -> (usize, usize) {
        let position = self.position;
        while Lexer::is_letter(self.ch) {
            self.read_char();
        }
        (position, self.position)
    }

    fn read_number(&mut self) -> (usize, usize) {
        let position = self.position;
        while Lexer::is_digit(self.ch) {
            self.read_char();
        }
        (position, self.position)
    }

    pub fn next_token(&mut self) -> Token {
        let token: Token;
        self.skip_whitespace();
        match self.ch as char {
            '=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    token =
                        self.create_token(TokenType::Equal, self.position - 1, self.read_position);
                } else {
                    token = self.create_token(TokenType::Assign, self.position, self.read_position);
                }
            }
            '!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    token = self.create_token(
                        TokenType::Notequal,
                        self.position - 1,
                        self.read_position,
                    );
                } else {
                    token = self.create_token(TokenType::Bang, self.position, self.read_position);
                }
            }
            ';' => {
                token = self.create_token(TokenType::Semicolon, self.position, self.read_position);
            }
            '(' => {
                token = self.create_token(TokenType::Lparen, self.position, self.read_position);
            }
            ')' => {
                token = self.create_token(TokenType::Rparen, self.position, self.read_position);
            }
            ',' => {
                token = self.create_token(TokenType::Comma, self.position, self.read_position);
            }
            '+' => {
                token = self.create_token(TokenType::Plus, self.position, self.read_position);
            }
            '-' => {
                token = self.create_token(TokenType::Minus, self.position, self.read_position);
            }
            '*' => {
                token = self.create_token(TokenType::Asterisk, self.position, self.read_position);
            }
            '/' => {
                token = self.create_token(TokenType::Slash, self.position, self.read_position);
            }
            '<' => {
                token = self.create_token(TokenType::Lessthan, self.position, self.read_position);
            }
            '>' => {
                token =
                    self.create_token(TokenType::Greaterthan, self.position, self.read_position);
            }
            '{' => {
                token = self.create_token(TokenType::Lbrace, self.position, self.read_position);
            }
            '}' => {
                token = self.create_token(TokenType::Rbrace, self.position, self.read_position);
            }
            '"' => {
                let (start, end) = self.read_string();
                token = self.create_token(TokenType::String, start, end);
            }
            '[' => {
                token = self.create_token(TokenType::Lbracket, self.position, self.read_position);
            }
            ']' => {
                token = self.create_token(TokenType::Rbracket, self.position, self.read_position);
            }
            '\u{0}' => token = self.create_eof(),
            _ => {
                if Lexer::is_letter(self.ch) {
                    let (start, end) = self.read_identifier();
                    let ident = &self.input[start..end];
                    return match ident {
                        "let" => self.create_token(TokenType::Let, start, end),
                        "fn" => self.create_token(TokenType::Function, start, end),
                        "true" => self.create_token(TokenType::True, start, end),
                        "false" => self.create_token(TokenType::False, start, end),
                        "if" => self.create_token(TokenType::If, start, end),
                        "else" => self.create_token(TokenType::Else, start, end),
                        "return" => self.create_token(TokenType::Return, start, end),
                        "while" => self.create_token(TokenType::While, start, end),
                        _ => self.create_token(TokenType::Ident, start, end),
                    };
                } else if Lexer::is_digit(self.ch) {
                    let (start, end) = self.read_number();
                    return self.create_token(TokenType::Int, start, end);
                } else {
                    token =
                        self.create_token(TokenType::Illegal, self.position, self.read_position);
                }
            }
        };
        self.read_char();
        token
    }

    pub fn initial_state(&mut self, src: &str) {
        self.input = src.to_string();
        self.position = 0;
        self.read_position = 0;
        self.line = 1;
        self.col = 0;
        self.ch = 0;
        self.read_char();
    }

    fn read_string(&mut self) -> (usize, usize) {
        let position = self.position + 1;
        loop {
            self.read_char();
            if self.ch == b'"' || self.ch == b'0' {
                break;
            }
        }
        (position, self.position)
    }
    pub fn tokens(&mut self) -> Vec<Token> {
        let mut token = Vec::new();
        while self.ch != 0 {
            token.push(self.next_token());
        }
        token.push(self.next_token());
        token
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lexer_init() {
        let src = String::from("let a;");
        let lexer = Lexer::new(src);
        let expected_lexer = Lexer {
            input: String::from("let a;"),
            read_position: 1,
            position: 0,
            line: 1,
            col: 0,
            ch: 'l' as u8,
        };

        assert_eq!(lexer, expected_lexer);
    }

    #[test]
    fn is_char() {
        assert_eq!(Lexer::is_letter('a' as u8), true);
        assert_eq!(Lexer::is_letter('_' as u8), true);
        assert_eq!(Lexer::is_letter('1' as u8), false);
        assert_eq!(Lexer::is_letter('[' as u8), false);
    }

    #[test]
    fn is_digit() {
        assert_eq!(Lexer::is_digit('1' as u8), true);
        assert_eq!(Lexer::is_digit('2' as u8), true);
        assert_eq!(Lexer::is_digit('a' as u8), false);
    }

    #[test]
    fn read_identifier() {
        let src = String::from("let");
        let mut lexer = Lexer::new(src);
        assert_eq!(lexer.next_token().token_type, TokenType::Let);
    }
}
