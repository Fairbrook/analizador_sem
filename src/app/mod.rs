use self::{
    analyzer::{error::AnalyzerError, graph::Graph, sintactic::Analyzer, symbols::SymbolsTable},
    tree::TreeItem,
};

pub mod analyzer;
pub mod tree;
pub struct App {
    pub input: String,
    last_state: AppResult,
    last_input: String,
}

pub type AppResult = Result<State, AnalyzerError>;

#[derive(Debug, Clone, Default)]
pub struct State {
    pub postfix: String,
    pub prefix: String,
    pub tree: TreeItem,
    pub symbols_table: SymbolsTable,
    pub graph: Graph,
}

impl App {
    pub fn run_analyzer(&mut self) -> AppResult {
        if self.last_input == self.input {
            return self.last_state.clone();
        }
        let mut analyzer = Analyzer::new(&self.input);
        let response = analyzer.analyze();
        if let Err(e) = response {
            self.last_input = self.input.clone();
            self.last_state = Err(e.clone());
            return Err(e);
        }
        let res = response?;
        self.last_state = Ok(State {
            prefix: res.prefix,
            postfix: res.postfix,
            tree: res.tree,
            symbols_table: analyzer.symbols_table,
            graph: analyzer.graph,
        });
        self.last_input = self.input.clone();
        return self.last_state.clone();
    }
}

impl Default for App {
    fn default() -> Self {
        App {
            input: String::from(""),
            last_input: String::from(""),
            last_state: Ok(State::default()),
        }
    }
}
