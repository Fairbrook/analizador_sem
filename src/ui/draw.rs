use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Spans,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{tree::TreeItem, App};

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

    if app.input.len() > 0 {
        let result = app.run_analyzer();
        match result {
            Ok(res) => {
                let result_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Percentage(50),
                            Constraint::Percentage(50),
                        ]
                        .as_ref(),
                    )
                    .split(main_layout[1]);
                let posfix_result = Paragraph::new(format!("Posfijo: {}", res.postfix))
                    .alignment(Alignment::Center);
                let prefix_result =
                    Paragraph::new(format!("Prefijo: {}", res.prefix)).alignment(Alignment::Center);
                let tree_paragraph = get_tree_paragraph(&res.tree).block(tree_block);
                f.render_widget(result_block, main_layout[1]);
                f.render_widget(posfix_result, result_layout[0]);
                f.render_widget(prefix_result, result_layout[1]);
                f.render_widget(tree_paragraph, main_layout[2]);
            }
            Err(err) => {
                err_str = format!("{}", err);
                result_block = result_block.border_style(Style::default().fg(Color::LightRed));
                let error_paragraph = Paragraph::new(err_str.as_ref()).block(result_block);
                f.render_widget(error_paragraph, main_layout[1]);
                f.render_widget(tree_block, main_layout[2])
            }
        };
        return ();
    }
    f.render_widget(result_block, main_layout[1]);
    f.render_widget(tree_block, main_layout[2])
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

fn get_tree_paragraph<'a>(tree: &'a TreeItem) -> Paragraph {
    Paragraph::new(get_tree_spans(tree, ""))
}
