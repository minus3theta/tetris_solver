use std::str::FromStr;

use thiserror::Error;

use super::Board;
use super::Location;

const EMPTY_ROW: u16 = 0b1110_0000_0000_0111;
const FILLED_ROW: u16 = 0b1111_1111_1111_1111;
const OFFSET_X: i8 = 3;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

fn get_bit_at(row: u16, x: i8) -> bool {
    row >> x & 1 != 0
}

fn set_bit_at(row: &mut u16, x: i8) {
    *row |= 1 << x;
}

fn unset_bit_at(row: &mut u16, x: i8) {
    *row &= !(1 << x);
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
        get_bit_at(row, x)
    }

    fn set(&mut self, location: Location, cell: Self::Cell) {
        let y = location.y as usize;
        if y >= C {
            return;
        }
        let row = &mut self.rows[y];
        let x = location.x + OFFSET_X;
        if cell {
            set_bit_at(row, x);
        } else {
            unset_bit_at(row, x);
        }
    }

    fn erase_filled_lines(&mut self) -> usize {
        let mut erased = 0;
        let mut y = 0;
        while y + erased < C {
            if self.rows[y] == FILLED_ROW {
                self.rows.copy_within(y + 1..C, y);
                erased += 1;
            } else {
                y += 1;
            }
        }
        self.rows[C - erased..C].fill(EMPTY_ROW);
        erased
    }
}

impl<const C: usize> std::fmt::Display for RowOrientedBitBoard<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &row in self.rows.iter().rev() {
            for x in OFFSET_X..super::BOARD_WIDTH as i8 + OFFSET_X {
                write!(f, "{}", if get_bit_at(row, x) { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum RowOrientedBitBoardFromStrError {
    #[error("Invalid character: '{0}'")]
    InvalidChar(char),
    #[error("Block height exceeds board ceiling ({0})")]
    ExceedBoardCeiling(usize),
    #[error("Invalid line width: `{0}`")]
    InvalidLineWidth(usize),
}

impl<const C: usize> FromStr for RowOrientedBitBoard<C> {
    type Err = RowOrientedBitBoardFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.trim().lines().rev();
        let mut board = Self::new();
        for row in board.rows.iter_mut() {
            let line = if let Some(line) = lines.next() {
                line
            } else {
                break;
            };
            if line.len() != super::BOARD_WIDTH {
                return Err(Self::Err::InvalidLineWidth(line.len()));
            }
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' | 'X' => set_bit_at(row, x as i8 + OFFSET_X),
                    '.' | '_' => (),
                    _ => return Err(Self::Err::InvalidChar(c)),
                }
            }
        }
        if lines.next().is_some() {
            return Err(Self::Err::ExceedBoardCeiling(C));
        }
        Ok(board)
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

    #[test]
    fn erase_empty() {
        let mut board = RowOrientedBitBoard::<4>::new();
        assert_eq!(board.erase_filled_lines(), 0);
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
    fn erase_all() {
        let mut board = RowOrientedBitBoard {
            rows: [FILLED_ROW; 4],
        };
        assert_eq!(board.erase_filled_lines(), 4);
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
    fn erase_mid() {
        let mut board = RowOrientedBitBoard::<4>::from_str(
            "\
##########
.......###
##########
###.......
",
        )
        .unwrap();
        assert_eq!(board.erase_filled_lines(), 2);
        assert_eq!(
            board.to_string(),
            "\
..........
..........
.......###
###.......
"
        );
    }

    #[test]
    fn from_string_success() {
        let input = "\
#.....####
##....####
###...####
##....####
";
        assert_eq!(
            RowOrientedBitBoard::<4>::from_str(input)
                .unwrap()
                .to_string(),
            input
        );
    }

    #[test]
    fn from_string_invalid_char() {
        let input = "\
#.....####
##....#A##
###...####
##....####
";
        assert_eq!(
            RowOrientedBitBoard::<4>::from_str(input),
            Err(RowOrientedBitBoardFromStrError::InvalidChar('A'))
        );
    }

    #[test]
    fn from_string_exceed_board_ceiling() {
        let input = "\
..........
#.....####
##....####
###...####
##....####
";
        assert_eq!(
            RowOrientedBitBoard::<4>::from_str(input),
            Err(RowOrientedBitBoardFromStrError::ExceedBoardCeiling(4))
        );
    }

    #[test]
    fn from_string_invalid_line_width() {
        let input = "\
#.....####
##....####
###...###
##....####
";
        assert_eq!(
            RowOrientedBitBoard::<4>::from_str(input),
            Err(RowOrientedBitBoardFromStrError::InvalidLineWidth(9))
        );
    }
}
