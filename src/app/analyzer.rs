use std::{char, error, fmt};

use super::tree::TreeItem;

#[derive(Debug, Clone, Default)]
pub struct AnalyzerError {
    character: char,
    expected: String,
}
impl AnalyzerError {
    pub fn new(character: char, expected: String) -> Self {
        AnalyzerError {
            character,
            expected,
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

#[derive(Debug, Clone, Default)]
pub struct Analyzed {
    pub result: f32,
    pub postfix: String,
    pub prefix: String,
    pub tree: TreeItem,
}

pub type AnalyzerResult = Result<Analyzed, AnalyzerError>;

#[derive(Debug, Clone)]
pub struct Analyzer {
    input: String,
    current: char,
}

impl Analyzer {
    pub fn new(input: &String) -> Self {
        let mut input_copy = input.clone();
        let current = input_copy.remove(0);
        Analyzer {
            input: input_copy,
            current,
        }
    }

    pub fn check_and_next(&mut self, character: char) -> Result<(), AnalyzerError> {
        if self.current != character {
            return Err(AnalyzerError::new(self.current, String::from(character)));
        }
        if self.input.len() > 0 {
            self.current = self.input.remove(0);
        } else {
            self.current = '\0';
        }
        Ok(())
    }

    pub fn analyze(&mut self) -> AnalyzerResult {
        let res = self.expr();
        if self.current != '\0' {
            return Err(AnalyzerError::new(self.current, String::from('\0')));
        }
        res
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
        match self.current {
            '+' => {
                self.check_and_next('+')?;
                let term = self.term()?;
                let mut partial = Analyzed {
                    result: analyzed.result + term.result,
                    postfix: format!("{} {} +", analyzed.postfix, term.postfix),
                    prefix: format!("+ {} {}", analyzed.prefix, term.prefix),
                    tree: TreeItem {
                        root,
                        items: vec![TreeItem::new("+".to_string()), term.tree],
                    },
                };
                let mut res = self.rest_expr(&partial)?;
                partial.tree.items.push(res.tree);
                res.tree = partial.tree;
                Ok(res)
            }
            '-' => {
                self.check_and_next('-')?;
                let term = self.term()?;
                let mut partial = Analyzed {
                    result: analyzed.result - term.result,
                    postfix: format!("{} {} -", analyzed.postfix, term.postfix),
                    prefix: format!("- {} {} ", analyzed.prefix, term.prefix),
                    tree: TreeItem {
                        root,
                        items: vec![TreeItem::new("-".to_string()), term.tree],
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
                    items: vec![TreeItem::new(String::from("ε"))],
                };
                Ok(res)
            }
        }
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
        let root = "rest_term".to_string();
        match self.current {
            '*' => {
                self.check_and_next('*')?;
                let factor = self.factor()?;
                let mut partial = Analyzed {
                    result: analyzed.result * factor.result,
                    postfix: format!("{} {} *", analyzed.postfix, factor.postfix),
                    prefix: format!("* {} {} ", analyzed.prefix, factor.prefix),
                    tree: TreeItem {
                        root,
                        items: vec![TreeItem::new("*".to_string()), factor.tree],
                    },
                };
                let mut res = self.rest_term(&partial)?;
                partial.tree.items.push(res.tree);
                res.tree = partial.tree;
                Ok(res)
            }
            '/' => {
                self.check_and_next('/')?;
                let factor = self.factor()?;
                let mut partial = Analyzed {
                    result: analyzed.result / factor.result,
                    postfix: format!("{} {} /", analyzed.postfix, factor.postfix),
                    prefix: format!("/ {} {}", analyzed.prefix, factor.prefix),
                    tree: TreeItem {
                        root,
                        items: vec![TreeItem::new("/".to_string()), factor.tree],
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
                    items: vec![TreeItem::new(String::from("ε"))],
                };
                Ok(res)
            }
        }
    }

    pub fn factor(&mut self) -> AnalyzerResult {
        if self.current == '(' {
            self.check_and_next('(')?;
            let mut analyzed = self.expr()?;
            self.check_and_next(')')?;
            analyzed.tree = TreeItem {
                root: String::from("factor"),
                items: vec![
                    TreeItem::new("(".to_string()),
                    analyzed.tree,
                    TreeItem::new(")".to_string()),
                ],
            };
            return Ok(analyzed);
        }
        let mut factor = self.digit()?;
        factor.tree = TreeItem {
            root: String::from("factor"),
            items: vec![factor.tree],
        };
        Ok(factor)
    }

    pub fn digit(&mut self) -> AnalyzerResult {
        let parsed = match self.current.to_digit(10) {
            Some(digit) => digit,
            None => {
                return Err(AnalyzerError::new(
                    self.current,
                    String::from("digito (0..9)"),
                ))
            }
        };
        let analyzed = Analyzed {
            result: parsed as f32,
            postfix: self.current.to_string(),
            prefix: self.current.to_string(),
            tree: TreeItem {
                root: "digit".to_string(),
                items: vec![TreeItem::new(self.current.to_string())],
            },
        };
        self.check_and_next(self.current)?;
        Ok(analyzed)
    }
}
