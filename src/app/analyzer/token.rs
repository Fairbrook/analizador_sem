#[derive(Debug, Clone, Default, PartialEq)]
pub enum TokenType {
    Number,
    Id,
    Plus,
    Minus,
    Asterisk,
    Slash,
    OpenParenthesis,
    ClosingParenthesis,
    #[default]
    EOF,
}

#[derive(Debug, Clone, Default)]
pub struct Token {
    pub position: u32,
    pub lexeme: String,
    pub token_type: TokenType,
}

pub fn type_to_string(t: &TokenType) -> String {
    match t {
        TokenType::ClosingParenthesis => String::from(")"),
        TokenType::OpenParenthesis => String::from("("),
        TokenType::Asterisk => String::from("*"),
        TokenType::Slash => String::from("/"),
        TokenType::Plus => String::from("+"),
        TokenType::Minus => String::from("-"),
        TokenType::Number => String::from("número"),
        TokenType::Id => String::from("identificador"),
        TokenType::EOF => String::from("Final de archivo"),
    }
}
