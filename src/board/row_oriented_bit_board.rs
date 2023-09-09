use super::Board;
use super::Location;

const EMPTY_ROW: u16 = 0b1110_0000_0000_0111;
const FILLED_ROW: u16 = 0b1111_1111_1111_1111;

#[derive(Debug, Clone)]
pub struct RowOrientedBitBoard<const C: usize> {
    rows: [u16; C],
}

impl<const C: usize> RowOrientedBitBoard<C> {
    pub fn new() -> Self {
        Self {
            rows: [EMPTY_ROW; C],
        }
    }
}

impl<const C: usize> Default for RowOrientedBitBoard<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const C: usize> Board for RowOrientedBitBoard<C> {
    type Cell = bool;

    fn ceil(&self) -> usize {
        C
    }

    fn get(&self, location: Location) -> Self::Cell {
        todo!()
    }

    fn set(&mut self, location: Location, cell: Self::Cell) {
        todo!()
    }
}
