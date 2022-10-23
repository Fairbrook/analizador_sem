use std::{error, fmt};

#[derive(Debug, Clone, Default)]
pub struct AnalyzerError {
    character: String,
    pos: u32,
    expected: String,
}
impl AnalyzerError {
    pub fn new(character: &str, pos: u32, expected: &str) -> Self {
        AnalyzerError {
            character: String::from(character),
            expected: String::from(expected),
            pos,
        }
    }
}
impl error::Error for AnalyzerError {}
impl fmt::Display for AnalyzerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Caracter inesperado '{}', en la posición {}, se esperaba: '{}'",
            self.character, self.pos, self.expected
        )
    }
}
