use crate::{error::LuxtError, token::Token};

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
            self.scan_token();
        }

        Token::eof(self.line);
        Ok(&self.tokens)
    }

    fn scan_token(&mut self) {
        todo!()
    }
}
