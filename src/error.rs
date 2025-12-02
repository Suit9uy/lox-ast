#[derive(Debug)]
pub struct LoxError {
    line: usize,
    message: String,
}

impl LoxError {
    pub fn error(line: usize, message: String) -> Self {
        LoxError { line, message }
    }

    pub fn report(self, lox: String) {
        eprintln!("[line {}] Error{}: {}", self.line, lox, self.message);
    }
}
