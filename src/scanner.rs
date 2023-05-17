use crate::{error::LuxtError, token::Token, token_type::TokenType};

pub struct Scanner<'a> {
    source: &'a [u8],
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a [u8]) -> Scanner<'a> {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LuxtError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token::eof(self.line));
        Ok(&self.tokens)
    }

    fn advance(&mut self) -> u8 {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn add_token(&mut self, token_type: TokenType) {
        // TODO(Stephen): I should probably do some error handling here...
        let lexeme = std::str::from_utf8(&self.source[self.start..self.current])
            .unwrap()
            .to_string();
        self.tokens.push(Token::new(token_type, lexeme, self.line));
    }

    fn match_next(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            return false;
        }

        let next_char = self.source[self.current];
        if next_char != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn scan_token(&mut self) -> Result<(), LuxtError> {
        let c: u8 = self.advance();
        match c {
            b'(' => self.add_token(TokenType::LeftParen),
            b')' => self.add_token(TokenType::RightParen),
            b'{' => self.add_token(TokenType::LeftBrace),
            b'}' => self.add_token(TokenType::RightBrace),
            b',' => self.add_token(TokenType::Comma),
            b'.' => self.add_token(TokenType::Dot),
            b'-' => self.add_token(TokenType::Minus),
            b'+' => self.add_token(TokenType::Plus),
            b';' => self.add_token(TokenType::SemiColon),
            b'*' => self.add_token(TokenType::Star),
            b'!' => {
                let t = match self.match_next(b'=') {
                    true => TokenType::BangEqual,
                    false => TokenType::Bang,
                };
                self.add_token(t);
            }
            b'=' => {
                let t = match self.match_next(b'=') {
                    true => TokenType::EqualEqual,
                    false => TokenType::Equal,
                };
                self.add_token(t);
            }
            b'>' => {
                let t = match self.match_next(b'=') {
                    true => TokenType::GreaterEqual,
                    false => TokenType::Greater,
                };
                self.add_token(t);
            }
            b'<' => {
                let t = match self.match_next(b'=') {
                    true => TokenType::LessEqual,
                    false => TokenType::Less,
                };
                self.add_token(t);
            }
            _ => {
                // this should be fixed...
                panic!("Error getting character in scan_token: {}", c == b'\n');
            }
        };
        Ok(())
    }
}
