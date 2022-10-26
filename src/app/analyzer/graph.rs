use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

use super::token::TokenType;

#[derive(Debug, Clone, Hash)]
pub struct Node {
    pub op: TokenType,
    pub is_leaf: bool,
    pub left: u64,
    pub right: u64,
}

#[derive(Debug, Clone)]
pub struct NodeWithIndex {
    pub node: Node,
    pub index: usize,
}

#[derive(Debug, Clone, Default)]
pub struct Graph {
    pub table: HashMap<u64, NodeWithIndex>,
    pub stack: Vec<u64>,
}

impl NodeWithIndex {
    pub fn new(node: Node, index: usize) -> Self {
        NodeWithIndex { index, node }
    }
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            table: HashMap::new(),
            stack: Vec::new(),
        }
    }
    pub fn add(&mut self, node: Node) -> u64 {
        let mut hasher = DefaultHasher::new();
        node.hash(&mut hasher);
        let hash = hasher.finish();
        if let None = self.table.get(&hash) {
            let node_with_index = NodeWithIndex::new(node.clone(), self.stack.len());
            self.table.insert(hash, node_with_index);
            self.stack.push(hash);
        }
        hash
    }

    pub fn get(&self, hash: &u64) -> Option<&NodeWithIndex> {
        self.table.get(hash)
    }
}
