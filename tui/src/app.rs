use crossterm::event::{Event, KeyCode};
use ratatui::Frame;

use crate::game_widget::GameWidget;

pub struct App {
    // TODO Offer larger board sizes
    game: GameWidget<9>,

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
                    KeyCode::Esc => {
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
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        if let Err(e) = self.game.place_stone() {
                            // TODO Show error and/or play sound
                        }
                    }
                    _ => (),
                }
            }
            _ => {}
        }
    }

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(&self.game, frame.area())
    }
}
