use std::{error, fmt};

#[derive(Debug, Clone, Default)]
pub struct AnalyzerError {
    character: String,
    expected: String,
}
impl AnalyzerError {
    pub fn new(character: &str, expected: &str) -> Self {
        AnalyzerError {
            character: String::from(character),
            expected: String::from(expected),
        }
    }
}
impl error::Error for AnalyzerError {}
impl fmt::Display for AnalyzerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Caracter inesperado '{}', se esperaba: '{}'",
            self.character, self.expected
        )
    }
}
