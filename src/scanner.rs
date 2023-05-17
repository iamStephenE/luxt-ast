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

    // maybe too idiomatic? lol
    fn peek(&self) -> u8 {
        match self.is_at_end() {
            true => return b'\0',
            false => self.source[self.current],
        }
    }

    fn peek_next(&self) -> u8 {
        match self.current + 1 >= self.source.len() {
            true => return b'\0',
            false => self.source[self.current + 1],
        }
    }

    fn string(&mut self) {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            panic!("Unterminated string.");
        }

        self.advance(); // the closing "

        let value = std::str::from_utf8(&self.source[self.start + 1..self.current - 1])
            .unwrap()
            .to_string();
        self.add_token(TokenType::String(value));
    }

    fn is_digit(&self, c: u8) -> bool {
        c >= b'0' && c <= b'9'
    }

    fn is_alpha(&self, c: u8) -> bool {
        c >= b'a' && c <= b'z' || c >= b'A' && c <= b'Z' || c == b'_'
    }

    fn is_alpha_numeric(&self, c: u8) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        // look for fractional part.
        if self.peek() == b'.' && self.is_digit(self.peek_next()) {
            // consume the "."
            self.advance();
        }

        while self.is_digit(self.peek()) {
            self.advance();
        }

        let lexeme = std::str::from_utf8(&self.source[self.start..self.current])
            .unwrap()
            .to_string();
        let value = lexeme.parse::<f64>().unwrap();
        self.add_token(TokenType::Number(value));
    }

    fn get_lexeme_token(&self, lexeme: &str) -> TokenType {
        match lexeme {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier(lexeme.to_string()),
        }
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        // here we need to determine if the lexeme is a keyword
        // if its not then it is an identifier
        let lexeme = std::str::from_utf8(&self.source[self.start..self.current]).unwrap();
        let lexeme_token = self.get_lexeme_token(lexeme);
        self.add_token(lexeme_token);
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
            b'/' => {
                if self.match_next(b'/') {
                    while self.peek() != b'\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            b' ' | b'\r' | b'\t' => {
                // ignore white space
            }
            b'\n' => self.line += 1,
            b'"' => self.string(),
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    panic!("Unexpected character: {}", c);
                }
            }
        };
        Ok(())
    }
}
