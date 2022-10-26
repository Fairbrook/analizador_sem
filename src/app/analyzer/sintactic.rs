use super::{
    error::AnalyzerError,
    graph::{Graph, Node},
    lexic::LexicAnalyzer,
    symbols::SymbolsTable,
    token::{type_to_string, TokenType},
};
use crate::app::tree::TreeItem;

#[derive(Debug, Clone, Default)]
pub struct Analyzed {
    pub postfix: String,
    pub prefix: String,
    pub node_hash: u64,
    pub tree: TreeItem,
}

pub type AnalyzerResult = Result<Analyzed, AnalyzerError>;

#[derive(Debug, Clone)]
pub struct Analyzer {
    lexic: LexicAnalyzer,
    pub symbols_table: SymbolsTable,
    pub graph: Graph,
}

impl Analyzer {
    pub fn new(input: &String) -> Self {
        Analyzer {
            lexic: LexicAnalyzer::new(input),
            symbols_table: SymbolsTable::new(),
            graph: Graph::new(),
        }
    }

    pub fn analyze(&mut self) -> AnalyzerResult {
        self.lexic.consume_token();
        let res = self.expr()?;
        self.lexic.check_and_next(&TokenType::EOF)?;
        Ok(res)
    }

    pub fn expr(&mut self) -> AnalyzerResult {
        let term = self.term()?;
        let mut res = self.rest_expr(&term)?;
        res.tree = TreeItem {
            root: "expr".to_string(),
            items: vec![term.tree, res.tree],
        };
        Ok(res)
    }

    pub fn rest_expr(&mut self, analyzed: &Analyzed) -> AnalyzerResult {
        let root = "rest_expr".to_string();
        if let Some(token) = self.lexic.token.clone() {
            return match token.token_type {
                TokenType::Plus | TokenType::Minus => {
                    self.lexic.consume_token();
                    let term = self.term()?;
                    let node_hash = self.graph.add(Node {
                        op: token.token_type.clone(),
                        left: analyzed.node_hash,
                        right: term.node_hash,
                        is_leaf: false,
                    });
                    let mut partial = Analyzed {
                        postfix: format!("{} {} {}", analyzed.postfix, term.postfix, token.lexeme),
                        prefix: format!("{} {} {}", token.lexeme, analyzed.prefix, term.prefix),
                        node_hash,
                        tree: TreeItem {
                            root,
                            items: vec![TreeItem::new(&token.lexeme), term.tree],
                        },
                    };
                    let mut res = self.rest_expr(&partial)?;
                    partial.tree.items.push(res.tree);
                    res.tree = partial.tree;
                    Ok(res)
                }
                _ => {
                    let mut res = analyzed.clone();
                    res.tree = TreeItem {
                        root,
                        items: vec![TreeItem::new("ε")],
                    };
                    Ok(res)
                }
            };
        }
        Ok(analyzed.clone())
    }

    pub fn term(&mut self) -> AnalyzerResult {
        let factor = self.factor()?;
        let mut res = self.rest_term(&factor)?;
        res.tree = TreeItem {
            root: "term".to_string(),
            items: vec![factor.tree, res.tree],
        };
        Ok(res)
    }

    pub fn rest_term(&mut self, analyzed: &Analyzed) -> AnalyzerResult {
        let root = String::from("rest_term");
        if let Some(token) = self.lexic.token.clone() {
            return match token.token_type {
                TokenType::Asterisk | TokenType::Slash => {
                    self.lexic.consume_token();
                    let factor = self.factor()?;
                    let node_hash = self.graph.add(Node {
                        op: token.token_type,
                        left: analyzed.node_hash,
                        right: factor.node_hash,
                        is_leaf: false,
                    });
                    let mut partial = Analyzed {
                        node_hash,
                        postfix: format!(
                            "{} {} {}",
                            analyzed.postfix, factor.postfix, token.lexeme
                        ),
                        prefix: format!("{} {} {} ", token.lexeme, analyzed.prefix, factor.prefix),
                        tree: TreeItem {
                            root,
                            items: vec![TreeItem::new(&token.lexeme), factor.tree],
                        },
                    };
                    let mut res = self.rest_term(&partial)?;
                    partial.tree.items.push(res.tree);
                    res.tree = partial.tree;
                    Ok(res)
                }
                _ => {
                    let mut res = analyzed.clone();
                    res.tree = TreeItem {
                        root,
                        items: vec![TreeItem::new("ε")],
                    };
                    Ok(res)
                }
            };
        }
        Ok(analyzed.clone())
    }

    pub fn factor(&mut self) -> AnalyzerResult {
        let root = String::from("factor");
        if let Some(token) = self.lexic.token.clone() {
            return match token.token_type {
                TokenType::OpenParenthesis => {
                    self.lexic.consume_token();
                    let mut analyzed = self.expr()?;
                    self.lexic.check_and_next(&TokenType::ClosingParenthesis)?;
                    analyzed.tree = TreeItem {
                        root,
                        items: vec![TreeItem::new("("), analyzed.tree, TreeItem::new(")")],
                    };
                    Ok(analyzed)
                }
                TokenType::Number | TokenType::Id => {
                    self.lexic.consume_token();
                    let symbol_hash = self.symbols_table.add(&token);
                    let node_hash = self.graph.add(Node {
                        op: token.token_type.clone(),
                        left: symbol_hash,
                        right: 0,
                        is_leaf: true,
                    });
                    Ok(Analyzed {
                        prefix: token.lexeme.clone(),
                        postfix: token.lexeme.clone(),
                        node_hash,
                        tree: TreeItem {
                            root,
                            items: vec![TreeItem {
                                root: type_to_string(&token.token_type),
                                items: vec![TreeItem::new(&token.lexeme)],
                            }],
                        },
                    })
                }
                _ => Err(AnalyzerError::new(
                    &token.lexeme,
                    token.position + 1,
                    "(, número o identificador",
                )),
            };
        }
        Err(AnalyzerError::new(
            &String::from(self.lexic.current),
            self.lexic.pos,
            "",
        ))
    }
}
