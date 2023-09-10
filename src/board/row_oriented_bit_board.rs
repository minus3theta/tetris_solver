use super::Board;
use super::Location;

const EMPTY_ROW: u16 = 0b1110_0000_0000_0111;
const FILLED_ROW: u16 = 0b1111_1111_1111_1111;
const OFFSET_X: i8 = 3;

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

fn bit_at(row: u16, x: i8) -> bool {
    row >> x & 1 != 0
}

impl<const C: usize> Default for RowOrientedBitBoard<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const C: usize> Board for RowOrientedBitBoard<C> {
    /// false: empty cell, true: filled cell
    type Cell = bool;

    fn ceil(&self) -> usize {
        C
    }

    fn get(&self, location: Location) -> Self::Cell {
        let y = location.y as usize;
        if location.y < 0 {
            return true;
        }
        let row = if y < C { self.rows[y] } else { EMPTY_ROW };
        let x = location.x + OFFSET_X;
        bit_at(row, x)
    }

    fn set(&mut self, location: Location, cell: Self::Cell) {
        let y = location.y as usize;
        if y >= C {
            return;
        }
        let row = &mut self.rows[y];
        let x = location.x + OFFSET_X;
        if cell {
            *row |= 1 << x;
        } else {
            *row &= !(1 << x);
        }
    }
}

impl<const C: usize> std::fmt::Display for RowOrientedBitBoard<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &row in self.rows.iter().rev() {
            for x in (OFFSET_X..super::BOARD_WIDTH as i8 + OFFSET_X).rev() {
                write!(f, "{}", if bit_at(row, x) { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display_empty_board() {
        let board = RowOrientedBitBoard::<4>::new();
        assert_eq!(
            board.to_string(),
            "\
..........
..........
..........
..........
"
        );
    }

    #[test]
    fn display_filled_board() {
        let board = RowOrientedBitBoard {
            rows: [FILLED_ROW; 4],
        };
        assert_eq!(
            board.to_string(),
            "\
##########
##########
##########
##########
"
        );
    }
}
