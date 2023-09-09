use super::location::Location;

pub trait Board: Clone {
    type Cell;

    fn ceil(&self) -> usize;
    fn get(&self, location: Location) -> Self::Cell;
    fn set(&mut self, location: Location, cell: Self::Cell) -> Self::Cell;
}
