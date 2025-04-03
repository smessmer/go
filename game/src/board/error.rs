use derive_more::{Display, Error};

#[derive(Error, Display, Debug)]
pub enum PlaceStoneError {
    #[display("Location already occupied")]
    CellOccupied,
}
