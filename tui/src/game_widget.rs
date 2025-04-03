use go_game::{Game, Player};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use crate::board_widget::BoardWidget;

pub struct GameWidget<const BoardSize: usize>
where
    [(); bitvec::mem::elts::<usize>(2 * BoardSize * BoardSize)]:,
{
    game: Game<BoardSize>,
    current_pos: (usize, usize),
}

impl<const BoardSize: usize> GameWidget<BoardSize>
where
    [(); bitvec::mem::elts::<usize>(2 * BoardSize * BoardSize)]:,
{
    pub fn new() -> Self {
        Self {
            game: Game::new(),
            current_pos: (0, 0),
        }
    }

    pub fn move_right(&mut self) {
        if self.current_pos.0 < BoardSize - 1 {
            self.current_pos.0 += 1;
        }
    }

    pub fn move_left(&mut self) {
        if self.current_pos.0 > 0 {
            self.current_pos.0 -= 1;
        }
    }

    pub fn move_up(&mut self) {
        if self.current_pos.1 > 0 {
            self.current_pos.1 -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.current_pos.1 < BoardSize - 1 {
            self.current_pos.1 += 1;
        }
    }

    pub fn place_stone(&mut self) -> Result<(), go_game::PlaceStoneError> {
        self.game
            .place_stone(self.current_pos.0, self.current_pos.1)
    }
}

impl<const BoardSize: usize> Widget for &GameWidget<BoardSize>
where
    [(); bitvec::mem::elts::<usize>(2 * BoardSize * BoardSize)]:,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Go Board ".bold());
        let block = Block::bordered()
            .title(title)
            .borders(ratatui::widgets::Borders::ALL)
            .style(ratatui::style::Style::default().fg(ratatui::style::Color::White));
        let inner_area = block.inner(area);
        let board = BoardWidget {
            board: self.game.board(),
            current_pos: self.current_pos,
        };
        board.render(inner_area, buf);
        let player_text = Text::from(vec![Line::from(vec![
            "Turn: ".into(),
            player_name(self.game.current_player()).yellow(),
        ])]);
        Paragraph::new(player_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

fn player_name(player: Player) -> &'static str {
    match player {
        Player::Black => "Black",
        Player::White => "White",
    }
}
