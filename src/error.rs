use core::fmt;
// -----------------------------------------------------------------------
// Mechanism for error handling, maybe look into making this better later

#[derive(Debug)]
pub struct CodeLocation {
    line: usize,
    offset: usize,
}

impl CodeLocation {
    pub fn new(line: usize, offset: usize) -> CodeLocation {
        CodeLocation { line, offset }
    }
}

impl fmt::Display for CodeLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[line {}, offset {}]", self.line, self.offset)
    }
}

#[derive(Debug)]
pub enum LuxtError {
    InvalidUtf8Character { location: CodeLocation },
    UnterminatedString { location: CodeLocation },
}

impl fmt::Display for LuxtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LuxtError::InvalidUtf8Character { location } => {
                write!(f, "[ERROR]: Invalid UTF-8 character at {}.", location)
            }
            LuxtError::UnterminatedString { location } => {
                write!(f, "[ERROR]: Unterminated string quote at {}.", location)
            }
        }
    }
}
