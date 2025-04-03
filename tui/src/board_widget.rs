use go_game::Board;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    text::{Line, Text},
    widgets::{Paragraph, Widget},
};

pub struct BoardWidget<'a, const BOARD_SIZE: usize>
where
    [(); bitvec::mem::elts::<usize>(2 * BOARD_SIZE * BOARD_SIZE)]:,
{
    pub board: &'a Board<BOARD_SIZE>,
    pub current_pos: (usize, usize),
}

impl<'a, const BOARD_SIZE: usize> Widget for &BoardWidget<'a, BOARD_SIZE>
where
    [(); bitvec::mem::elts::<usize>(2 * BOARD_SIZE * BOARD_SIZE)]:,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = (0..BOARD_SIZE)
            .map(|y| {
                Line::from(
                    (0..BOARD_SIZE)
                        .map(|x| {
                            let is_current_pos = self.current_pos == (x, y);
                            let cell = self.board[(x, y)];
                            let cell_str = match cell {
                                Some(go_game::Player::White) => "○ ", // white stone
                                Some(go_game::Player::Black) => "● ", // black stone
                                None => match (x, y) {
                                    (0, 0) => "┌─",                                               // top left corner
                                    (0, n) if n == BOARD_SIZE - 1 => "└─", // bottom left corner
                                    (n, 0) if n == BOARD_SIZE - 1 => "┐ ", // top right corner
                                    (n, m) if n == BOARD_SIZE - 1 && m == BOARD_SIZE - 1 => "┘ ", // bottom right corner
                                    (0, _) => "├─", // left edge
                                    (_, 0) => "┬─", // top edge
                                    (n, _) if n == BOARD_SIZE - 1 => "┤ ", // right edge
                                    (_, n) if n == BOARD_SIZE - 1 => "┴─", // bottom edge
                                    (_, _) => "┼─", // middle cell
                                },
                            };
                            if is_current_pos {
                                // TODO Only highlight the first character
                                cell_str.on_blue().bold()
                            } else {
                                cell_str.into()
                            }
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();
        Paragraph::new(Text::from(text)).render(area, buf);
    }
}
