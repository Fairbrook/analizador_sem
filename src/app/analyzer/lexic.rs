use super::{
    error::AnalyzerError,
    token::{type_to_string, Token, TokenType},
};

#[derive(Debug, Clone)]
pub struct LexicAnalyzer {
    pub current: char,
    pub input: String,
    pub pos: u32,
    pub token: Option<Token>,
}

pub type LexicResult = Result<Token, AnalyzerError>;

impl LexicAnalyzer {
    pub fn new(input: &String) -> Self {
        let mut input_copy = input.clone();
        let current = input_copy.remove(0);
        LexicAnalyzer {
            pos: 1,
            current,
            input: input_copy,
            token: None,
        }
    }

    pub fn check_and_next(&mut self, expected: &TokenType) -> LexicResult {
        if let Some(token) = self.token.clone() {
            if *expected != token.token_type {
                return Err(AnalyzerError::new(
                    &(token.lexeme),
                    token.position,
                    &type_to_string(expected),
                ));
            }
            self.consume_token();
            return Ok(token.clone());
        }
        Err(AnalyzerError::new(
            &String::from(self.current),
            self.pos,
            &type_to_string(expected),
        ))
    }

    pub fn consume_token(&mut self) -> Option<Token> {
        let mut token = Token {
            position: self.pos,
            lexeme: String::from(self.current),
            token_type: TokenType::EOF,
        };
        while self.current == ' ' {
            self.next_char();
        }
        if let Some(token) = self.single_char_token() {
            self.token = Some(token.clone());
            return Some(token);
        }
        match self.current {
            '0'..='9' => {
                token.token_type = TokenType::Number;
                if let Some(num) = self.number() {
                    token.lexeme = num;
                } else {
                    return None;
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                token.token_type = TokenType::Id;
                if let Some(id) = self.id() {
                    token.lexeme = id;
                } else {
                    return None;
                }
            }
            '\0' => (),
            _ => {
                token.token_type = TokenType::Unknown;
                token.lexeme = self.current.to_string();
            }
        };
        self.token = Some(token.clone());
        Some(token)
    }

    pub fn single_char_token(&mut self) -> Option<Token> {
        let mut token = Token {
            position: self.pos,
            lexeme: String::from(self.current),
            token_type: TokenType::EOF,
        };
        match self.current {
            '(' => token.token_type = TokenType::OpenParenthesis,
            ')' => token.token_type = TokenType::ClosingParenthesis,
            '-' => token.token_type = TokenType::Minus,
            '+' => token.token_type = TokenType::Plus,
            '/' => token.token_type = TokenType::Slash,
            '*' => token.token_type = TokenType::Asterisk,
            _ => {
                return None;
            }
        };
        self.next_char();
        return Some(token);
    }

    fn next_char(&mut self) {
        if self.input.len() > 0 {
            self.pos += 1;
            self.current = self.input.remove(0);
            return ();
        }
        self.current = '\0';
    }

    pub fn number(&mut self) -> Option<String> {
        if let Some(digit) = self.digit() {
            return Some(self.rest_num(&digit.to_string()));
        }
        None
    }

    pub fn rest_num(&mut self, prev: &str) -> String {
        if let Some(digit) = self.digit() {
            let mut next = String::from(prev);
            next.push(digit);
            return self.rest_num(&next);
        }
        return String::from(prev);
    }

    pub fn id(&mut self) -> Option<String> {
        if let Some(letter) = self.letter() {
            return Some(self.rest_id(&letter.to_string()));
        }
        if self.current == '_' {
            return Some(self.rest_id(&self.current.to_string()));
        }
        return None;
    }

    pub fn rest_id(&mut self, prev: &str) -> String {
        let mut next = String::from(prev);
        if let Some(letter) = self.letter() {
            next.push(letter);
            return self.rest_id(&next);
        }
        if self.current == '_' {
            next.push(self.current);
            self.next_char();
            return self.rest_id(&next);
        }
        if let Some(digit) = self.digit() {
            next.push(digit);
            return self.rest_id(&next);
        }
        String::from(prev)
    }

    pub fn letter(&mut self) -> Option<char> {
        let str = match self.current {
            'a'..='z' | 'A'..='Z' => self.current,
            _ => {
                return None;
            }
        };
        self.next_char();
        Some(str)
    }

    pub fn digit(&mut self) -> Option<char> {
        if !self.current.is_digit(10) {
            return None;
        }
        let res = self.current;
        self.next_char();
        Some(res)
    }
}
