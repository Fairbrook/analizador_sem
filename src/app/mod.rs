use self::{
    analyzer::sintactic::{Analyzed, Analyzer, AnalyzerResult},
    tree::TreeItem,
};

pub mod analyzer;
pub mod tree;
pub struct App {
    pub input: String,
    last_result: AnalyzerResult,
    last_input: String,
}

impl App {
    pub fn run_analyzer(&mut self) -> AnalyzerResult {
        if self.last_input == self.input {
            return self.last_result.clone();
        }
        let mut analyzer = Analyzer::new(&self.input);
        self.last_result = analyzer.analyze();
        self.last_input = self.input.clone();
        return self.last_result.clone();
    }
}

impl Default for App {
    fn default() -> Self {
        App {
            input: String::from(""),
            last_input: String::from(""),
            last_result: Ok(Analyzed {
                postfix: String::from(""),
                prefix: String::from(""),
                tree: TreeItem::new(""),
            }),
        }
    }
}
