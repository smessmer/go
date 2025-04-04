#![feature(generic_const_exprs)]

use color_eyre::Result;
use crossterm::event::{self};
use go_tui::App;
use ratatui::DefaultTerminal;

fn main() -> Result<()> {
    tui_logger::init_logger(log::LevelFilter::Trace).unwrap();
    tui_logger::set_default_level(log::LevelFilter::Info);

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
