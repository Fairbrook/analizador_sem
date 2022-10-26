use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

use super::token::Token;

#[derive(Debug, Clone)]
pub struct TokenWithIndex {
    pub token: Token,
    pub index: usize,
}

impl TokenWithIndex {
    pub fn new(token: Token, index: usize) -> Self {
        TokenWithIndex { token, index }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SymbolsTable {
    pub table: HashMap<u64, TokenWithIndex>,
    pub stack: Vec<u64>,
}

impl SymbolsTable {
    pub fn new() -> Self {
        SymbolsTable {
            table: HashMap::new(),
            stack: Vec::new(),
        }
    }
    pub fn add(&mut self, token: &Token) -> u64 {
        let mut hasher = DefaultHasher::new();
        token.hash(&mut hasher);
        let hash = hasher.finish();
        if let None = self.table.get(&hash) {
            let token_with_index = TokenWithIndex::new(token.clone(), self.stack.len());
            self.table.insert(hash, token_with_index);
            self.stack.push(hash);
        }
        hash
    }

    pub fn get(&self, hash: &u64) -> Option<&TokenWithIndex> {
        self.table.get(hash)
    }
}
