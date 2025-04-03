use go_game::Board;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::Stylize,
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
                        let cell_str = match cell {
                            Some(go_game::Player::White) => "⬤ ",
                            Some(go_game::Player::Black) => "◯ ",
                            None => "  ",
                        };
                        if is_current_pos {
                            cell_str.on_blue().bold()
                        } else {
                            cell_str.into()
                        }
                    })
                    .collect::<Vec<_>>(),
            )
        });
        let widths = [Constraint::Length(2); BoardSize];
        let table = Table::new(rows, widths);
        table.render(area, buf);
    }
}
