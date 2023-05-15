// -----------------------------------------------------------------------
// Mechanism for error handling, maybe look into making this better later

#[derive(Debug)]
pub struct LuxtError {
    line: usize,
    message: String,
}

impl LuxtError {
    #[allow(dead_code)]
    pub fn error(line: usize, message: String) -> LuxtError {
        LuxtError { line, message }
    }

    pub fn report(&self, loc: &str) {
        eprintln!("[line {}] Error{}: {}", self.line, loc, self.message)
    }
}
