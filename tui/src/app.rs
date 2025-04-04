use actually_beep::beep_with_hz_and_millis;
use crossterm::event::{Event, KeyCode};
use go_game::BoardSize9x9;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::Block;
use tui_logger::TuiLoggerWidget;

use crate::game_widget::GameWidget;

pub struct App {
    // TODO Offer larger board sizes
    game: GameWidget<BoardSize9x9>,

    should_exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            game: GameWidget::new(),
            should_exit: false,
        }
    }

    pub fn should_exit(&self) -> bool {
        self.should_exit
    }

    pub fn on_event(&mut self, event: Event) {
        match event {
            Event::Key(key) => {
                // Handle key events here, e.g. for quitting the app
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        self.should_exit = true;
                    }
                    KeyCode::Left => {
                        self.game.move_left();
                    }
                    KeyCode::Right => {
                        self.game.move_right();
                    }
                    KeyCode::Up => {
                        self.game.move_up();
                    }
                    KeyCode::Down => {
                        self.game.move_down();
                    }
                    KeyCode::Char('p') => {
                        let player = self.game.current_player();
                        self.game.pass_turn();
                        log::info!("{player}: pass turn");
                    }
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        let player = self.game.current_player();
                        let current_pos = self.game.current_pos();
                        match self.game.place_stone() {
                            Ok(()) => {
                                log::info!(
                                    // TODO Should the origin be bottom left or top left of the board?
                                    "{player}: placed stone at {}/{}",
                                    current_pos.0,
                                    current_pos.1
                                );
                            }
                            Err(e) => {
                                log::error!(
                                    // TODO Same here, which origin?
                                    "{player}: Failed to place stone at {}/{}: {:?}",
                                    current_pos.0,
                                    current_pos.1,
                                    e
                                );
                                beep_with_hz_and_millis(200, 75).unwrap();
                            }
                        }
                    }
                    _ => (),
                }
            }
            _ => {}
        }
    }

    pub fn draw(&self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
            .split(frame.area());
        frame.render_widget(&self.game, layout[0]);
        frame.render_widget(
            TuiLoggerWidget::default().block(Block::bordered().title("Log")),
            layout[1],
        );
    }
}
