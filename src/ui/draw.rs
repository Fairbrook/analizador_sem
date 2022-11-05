use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Spans,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{
    analyzer::{
        graph::{Graph, NodeWithIndex},
        symbols::SymbolsTable,
        token::type_to_string,
    },
    tree::TreeItem,
    App,
};

pub fn draw_frame<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let err_str: String;
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(f.size());
    let diagrams_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(30),
                Constraint::Percentage(30),
            ]
            .as_ref(),
        )
        .split(main_layout[2]);

    let input = Paragraph::new(app.input.as_ref()).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::LightCyan))
            .title("Entrada"),
    );

    f.render_widget(input, main_layout[0]);

    let instructions =
        Paragraph::new("Presione ESC para salir").style(Style::default().fg(Color::LightCyan));
    f.render_widget(instructions, main_layout[3]);

    let mut result_block = Block::default().borders(Borders::ALL).title("Resultado");
    let tree_block = Block::default().borders(Borders::ALL).title("Árbol");
    let symbols_table = Block::default()
        .borders(Borders::ALL)
        .title("Tabla de símbolos");
    let graph_block = Block::default().borders(Borders::ALL).title("Grafo");

    if app.input.len() > 0 {
        let result = app.run_analyzer();
        match result {
            Ok(res) => {
                let result_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Percentage(33),
                            Constraint::Percentage(33),
                            Constraint::Percentage(33),
                        ]
                        .as_ref(),
                    )
                    .split(main_layout[1]);
                let posfix_result = Paragraph::new(format!("Posfijo: {}", res.postfix))
                    .alignment(Alignment::Center);
                let prefix_result =
                    Paragraph::new(format!("Prefijo: {}", res.prefix)).alignment(Alignment::Center);
                let tree_paragraph = get_tree_paragraph(&res.tree).block(tree_block);
                let symbols_paragraph =
                    get_symbols_paragraph(&res.symbols_table).block(symbols_table);
                let get_graph_paragraph =
                    get_graph_paragraph(&res.graph, &res.symbols_table).block(graph_block);
                f.render_widget(result_block, main_layout[1]);
                f.render_widget(posfix_result, result_layout[0]);
                f.render_widget(prefix_result, result_layout[1]);
                f.render_widget(tree_paragraph, diagrams_layout[0]);
                f.render_widget(symbols_paragraph, diagrams_layout[1]);
                f.render_widget(get_graph_paragraph, diagrams_layout[2]);
                if let Some(num) = res.result {
                    let numeric_result = Paragraph::new(format!("Resultado: {}", num));
                    f.render_widget(numeric_result, result_layout[2]);
                }
            }
            Err(err) => {
                err_str = format!("{}", err);
                result_block = result_block.border_style(Style::default().fg(Color::LightRed));
                let error_paragraph = Paragraph::new(err_str.as_ref()).block(result_block);
                f.render_widget(error_paragraph, main_layout[1]);
                f.render_widget(tree_block, diagrams_layout[0]);
                f.render_widget(symbols_table, diagrams_layout[1]);
                f.render_widget(graph_block, diagrams_layout[2]);
            }
        };
        return ();
    }
    f.render_widget(result_block, main_layout[1]);
    f.render_widget(tree_block, diagrams_layout[0]);
    f.render_widget(symbols_table, diagrams_layout[1]);
    f.render_widget(graph_block, diagrams_layout[2]);
}

fn get_tree_spans<'a>(tree: &'a TreeItem, prepend: &str) -> Vec<Spans<'a>> {
    let mut span_str: String = format!("{}├ {}", prepend, tree.root);
    let mut next_prepend = prepend.to_owned() + "│  ";
    if prepend.len() == 0 {
        span_str = format!("{}", tree.root);
        next_prepend = prepend.to_owned() + " ";
    }
    let mut res = vec![Spans::from(span_str)];
    for item in tree.items.iter() {
        for span in get_tree_spans(&item, &next_prepend) {
            res.push(span);
        }
    }
    res
}

fn get_symbols_paragraph<'a>(table: &'a SymbolsTable) -> Paragraph {
    let mut spans = Vec::new();
    spans.push(Spans::from(""));
    spans.push(Spans::from(format!(
        "  ┌{:─>10}┬{:─>10}┬{:─>20}┐",
        "", "", ""
    )));
    spans.push(Spans::from(format!(
        "  │{: <10}│{: <10}│{: <20}│",
        "#", "Typo", "Lexema"
    )));
    spans.push(Spans::from(format!(
        "  ├{:─<10}┼{:─<10}┼{:─<20}┤",
        "", "", ""
    )));
    for (i, hash) in table.stack.iter().enumerate() {
        if let Some(entry) = table.get(hash) {
            let type_str = type_to_string(&entry.token.token_type);
            let lexeme = &entry.token.lexeme;
            let lexeme: String = lexeme.chars().into_iter().take(20).collect();
            let entry = format!("  │{: <10}│{: <10}│{: <20}│", i, type_str, lexeme);
            spans.push(Spans::from(entry));
            if i < table.stack.len() - 1 {
                spans.push(Spans::from(format!(
                    "  ├{:─<10}┼{:─<10}┼{:─<20}┤",
                    "", "", ""
                )));
            }
        }
    }
    spans.push(Spans::from(format!(
        "  └{:─<10}┴{:─<10}┴{:─<20}┘",
        "", "", ""
    )));
    return Paragraph::new(spans);
}

fn get_graph_paragraph<'a>(table: &'a Graph, symbols: &'a SymbolsTable) -> Paragraph<'a> {
    let mut spans = Vec::new();
    spans.push(Spans::from(""));
    spans.push(Spans::from(format!(
        "  ┌{:─>8}┬{:─>8}┬{:─>8}┬{:─>8}┐",
        "", "", "", ""
    )));
    spans.push(Spans::from(format!(
        "  │{: <8}│{: <8}│{: <8}│{: <8}│",
        "#", "Op", "Izq", "Der"
    )));
    spans.push(Spans::from(format!(
        "  ├{:─<8}┼{:─<8}┼{:─<8}┼{:─<8}┤",
        "", "", "", ""
    )));
    for (i, hash) in table.stack.iter().enumerate() {
        if let Some(entry) = table.get(hash) {
            let formated = get_node_string(&entry, symbols, table);
            spans.push(Spans::from(formated));
            if i < table.stack.len() - 1 {
                spans.push(Spans::from(format!(
                    "  ├{:─<8}┼{:─<8}┼{:─<8}┼{:─<8}┤",
                    "", "", "", ""
                )));
            }
        }
    }
    spans.push(Spans::from(format!(
        "  └{:─<8}┴{:─<8}┴{:─<8}┴{:─<8}┘",
        "", "", "", ""
    )));
    return Paragraph::new(spans);
}

fn get_node_string(
    node_with_index: &NodeWithIndex,
    symbols: &SymbolsTable,
    graph: &Graph,
) -> String {
    let i = node_with_index.index.to_string();
    let op_str = type_to_string(&node_with_index.node.op);
    let left = if node_with_index.node.is_leaf {
        if let Some(token) = symbols.get(&node_with_index.node.left) {
            format!(" ➔ {}", token.index)
        } else {
            String::new()
        }
    } else {
        if let Some(node) = graph.get(&node_with_index.node.left) {
            node.index.to_string()
        } else {
            String::new()
        }
    };
    let right = if node_with_index.node.is_leaf {
        String::new()
    } else {
        if let Some(node) = graph.get(&node_with_index.node.right) {
            node.index.to_string()
        } else {
            String::new()
        }
    };
    format!("  │{: <8}│{: <8}│{: <8}│{: <8}│", i, op_str, left, right)
}

fn get_tree_paragraph<'a>(tree: &'a TreeItem) -> Paragraph {
    Paragraph::new(get_tree_spans(tree, ""))
}
