use std::error;

use ui::{restore_terminal, run_app, setup_terminal};

pub mod app;
pub mod ui;

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut terminal = setup_terminal()?;
    let app = app::App::default();
    run_app(&mut terminal, app)?;
    restore_terminal(&mut terminal)?;

    Ok(())
}
