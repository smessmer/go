#![feature(generic_const_exprs)]

use color_eyre::Result;
use crossterm::event::{self};
use go_tui::App;
use ratatui::DefaultTerminal;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app = App::new();
    while !app.should_exit() {
        terminal.draw(|frame| app.draw(frame))?;
        app.on_event(event::read()?);
    }

    Ok(())
}
