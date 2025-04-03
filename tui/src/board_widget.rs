use go_game::Board;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    widgets::{Row, Table, Widget},
};

pub struct BoardWidget<'a, const BoardSize: usize>
where
    [(); bitvec::mem::elts::<usize>(2 * BoardSize * BoardSize)]:,
{
    pub board: &'a Board<BoardSize>,
    pub current_pos: (usize, usize),
}

impl<'a, const BoardSize: usize> Widget for &BoardWidget<'a, BoardSize>
where
    [(); bitvec::mem::elts::<usize>(2 * BoardSize * BoardSize)]:,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let rows = (0..BoardSize).map(|y| {
            Row::new(
                (0..BoardSize)
                    .map(|x| {
                        let is_current_pos = self.current_pos == (x, y);
                        let cell = self.board[(x, y)];
                        match (is_current_pos, cell) {
                            (false, Some(go_game::Player::White)) => "●",
                            (false, Some(go_game::Player::Black)) => "○",
                            (false, None) => " ",
                            (true, Some(go_game::Player::White)) => "●*",
                            (true, Some(go_game::Player::Black)) => "○*",
                            (true, None) => " *",
                        }
                    })
                    .collect::<Vec<&str>>(),
            )
        });
        let widths = [Constraint::Length(3); BoardSize];
        let table = Table::new(rows, widths);
        table.render(area, buf);
    }
}
