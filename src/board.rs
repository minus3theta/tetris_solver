use super::location::Location;

pub mod row_oriented_bit_board;

pub const BOARD_WIDTH: usize = 10;

pub trait Board: Clone {
    type Cell;

    fn ceil(&self) -> usize;
    fn get(&self, location: Location) -> Self::Cell;
    fn set(&mut self, location: Location, cell: Self::Cell);
}
