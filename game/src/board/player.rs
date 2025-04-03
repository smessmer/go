use enum_map::Enum;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Enum)]
pub enum Player {
    Black,
    White,
}

impl Player {
    #[inline]
    pub fn other_player(self) -> Self {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }
}
