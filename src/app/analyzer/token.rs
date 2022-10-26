use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone, Default, PartialEq, Hash)]
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
    Unknown,
    EOF,
}

#[derive(Debug, Clone, Default)]
pub struct Token {
    pub position: u32,
    pub lexeme: String,
    pub token_type: TokenType,
}

impl Hash for Token {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.lexeme.hash(state);
    }
}

impl Token {
    pub fn get_default_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
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
        TokenType::Id => String::from("id"),
        TokenType::EOF => String::from("Final de archivo"),
        TokenType::Unknown => String::from("desconocido"),
    }
}
