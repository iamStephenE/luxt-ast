use crate::{error::LuxtError, literal::Literal, token::Token, token_type::TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
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

    // helper methods
    fn advance(&mut self) -> char {
        // We are sure that we can unwrap here because we check the
        // is_at_end method first. I don't like this way of getting the
        // character but oh well for now.

        // TODO(Stephen): should probably use an iterator instead of a source
        // string but that complicates it and introduces lifetimes...
        let next_char = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        next_char
    }

    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        let next_char = self.source.chars().nth(self.current).unwrap();
        if next_char != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn scan_token(&mut self) -> Result<(), LuxtError> {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, Literal::Nil),
            ')' => self.add_token(TokenType::RightParen, Literal::Nil),
            '{' => self.add_token(TokenType::LeftBrace, Literal::Nil),
            '}' => self.add_token(TokenType::RightBrace, Literal::Nil),
            ',' => self.add_token(TokenType::Comma, Literal::Nil),
            '.' => self.add_token(TokenType::Dot, Literal::Nil),
            '-' => self.add_token(TokenType::Minus, Literal::Nil),
            '+' => self.add_token(TokenType::Plus, Literal::Nil),
            ';' => self.add_token(TokenType::SemiColon, Literal::Nil),
            '*' => self.add_token(TokenType::Star, Literal::Nil),
            '!' => {
                let t = match self.match_next('=') {
                    true => TokenType::BangEqual,
                    false => TokenType::Bang,
                };
                self.add_token(t, Literal::Nil);
            }
            '=' => {
                let t = match self.match_next('=') {
                    true => TokenType::EqualEqual,
                    false => TokenType::Equal,
                };
                self.add_token(t, Literal::Nil);
            }
            '>' => {
                let t = match self.match_next('=') {
                    true => TokenType::GreaterEqual,
                    false => TokenType::Greater,
                };
                self.add_token(t, Literal::Nil);
            }
            '<' => {
                let t = match self.match_next('=') {
                    true => TokenType::LessEqual,
                    false => TokenType::Less,
                };
                self.add_token(t, Literal::Nil);
            }
            _ => {
                // this should be fixed...
                panic!("Error getting character in scan_token: {}", c == '\n');
            }
        };
        Ok(())
    }
}
