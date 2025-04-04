use go_game::{BoardSize, Game, Player};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use crate::board_widget::BoardWidget;

pub struct GameWidget<BS: BoardSize>
where
    [(); bitvec::mem::elts::<usize>(2 * <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE)]:,
    [(); <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE]:,
{
    game: Game<BS>,
    current_pos: (usize, usize),
}

impl<BS: BoardSize> GameWidget<BS>
where
    [(); bitvec::mem::elts::<usize>(2 * <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE)]:,
    [(); <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE]:,
{
    pub fn new() -> Self {
        Self {
            game: Game::new(),
            current_pos: (0, 0),
        }
    }

    pub fn move_right(&mut self) {
        if self.current_pos.0 < <BS as BoardSize>::SIZE - 1 {
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
        if self.current_pos.1 < <BS as BoardSize>::SIZE - 1 {
            self.current_pos.1 += 1;
        }
    }

    pub fn place_stone(&mut self) -> Result<(), go_game::PlaceStoneError> {
        self.game
            .place_stone(self.current_pos.0, self.current_pos.1)
    }
}

impl<BS: BoardSize> Widget for &GameWidget<BS>
where
    [(); bitvec::mem::elts::<usize>(2 * <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE)]:,
    [(); <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE]:,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Go Board ".bold());
        let prisoner_text = Line::from(vec![
            "Prisoners Captured: ".into(),
            format!(
                "{}: {}",
                player_name(Player::Black),
                self.game.num_captured_by(Player::Black).into_usize()
            )
            .yellow(),
            " | ".into(),
            format!(
                "{}: {}",
                player_name(Player::White),
                self.game.num_captured_by(Player::White).into_usize()
            )
            .yellow(),
        ]);
        let block = Block::bordered()
            .title(title)
            .title_bottom(prisoner_text)
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
