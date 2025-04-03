use crossterm::event::{Event, KeyCode};
use go_game::Game;
use ratatui::Frame;

pub struct App {
    // TODO Offer larger board sizes
    game: Game<9>,

    should_exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            game: Game::new(),
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
                if key.code == KeyCode::Esc {
                    self.should_exit = true;
                }
            }
            _ => {}
        }
    }

    pub fn render(&self, frame: &mut Frame) {}
}
