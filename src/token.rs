use crate::token_type::TokenType;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub enum LiteralType {
    Number,
    String,
    Boolean,
    Null,
    Undefined,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: LiteralType,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: LiteralType, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    // adding eof token
    pub fn eof(line: usize) -> Token {
        Token::new(TokenType::Eof, String::new(), LiteralType::Null, line)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} {} {:?} at line {}",
            self.token_type, self.lexeme, self.literal, self.line
        )
    }
}
