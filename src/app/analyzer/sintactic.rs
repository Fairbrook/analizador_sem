use super::{
    error::AnalyzerError,
    lexic::{type_to_string, LexicAnalyzer, TokenType},
};
use crate::app::tree::TreeItem;

#[derive(Debug, Clone, Default)]
pub struct Analyzed {
    pub postfix: String,
    pub prefix: String,
    pub tree: TreeItem,
    pub result: Option<f32>,
}

pub type AnalyzerResult = Result<Analyzed, AnalyzerError>;

#[derive(Debug, Clone)]
pub struct Analyzer {
    lexic: LexicAnalyzer,
}

impl Analyzer {
    pub fn new(input: &String) -> Self {
        Analyzer {
            lexic: LexicAnalyzer::new(input),
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
                    let num = if let (Some(operand_a), Some(operand_b)) =
                        (analyzed.result, term.result)
                    {
                        match token.token_type {
                            TokenType::Plus => Some(operand_a + operand_b),
                            TokenType::Minus => Some(operand_a - operand_b),
                            _ => None,
                        }
                    } else {
                        None
                    };
                    let mut partial = Analyzed {
                        result: num,
                        postfix: format!("{} {} {}", analyzed.postfix, term.postfix, token.lexeme),
                        prefix: format!("{} {} {}", token.lexeme, analyzed.prefix, term.prefix),
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
                        items: vec![TreeItem::new("??")],
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
                    let num = if let (Some(operand_a), Some(operand_b)) =
                        (analyzed.result, factor.result)
                    {
                        match token.token_type {
                            TokenType::Asterisk => Some(operand_a * operand_b),
                            TokenType::Slash => Some(operand_a / operand_b),
                            _ => None,
                        }
                    } else {
                        None
                    };
                    let mut partial = Analyzed {
                        result: num,
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
                        items: vec![TreeItem::new("??")],
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
                    let num = if let TokenType::Number = token.token_type {
                        if let Ok(res) = token.lexeme.parse::<f32>() {
                            Some(res)
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    Ok(Analyzed {
                        prefix: token.lexeme.clone(),
                        postfix: token.lexeme.clone(),
                        result: num,
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
                    "(, n??mero o identificador",
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
