use crate::{token_type::TokenType};
use std::fmt;

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            line,
        }
    }

    // token
    pub fn eof(line: usize) -> Token {
        Token::new(TokenType::Eof, String::new(), line)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} {} at line {}",
            self.token_type, self.lexeme, self.line
        )
    }
}
