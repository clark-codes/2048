use std::fmt;


#[derive(Debug, Clone)]
pub struct Grid {
    board: [[u8; 4]; 4], // Grid size 4x4 with u8 for tile values.
}

impl Grid {
     pub fn new() -> Self {
        Grid { board: [[0; 4]; 4] } // Init an empty board
     }
     /// get a tile
     pub fn get(&self, row: usize, col: usize) -> u8 {
        self.board[row][col]
     }
     // set the value of a tile
     pub fn set(&mut self, row: usize, col: usize, value: u8) {
        self.board[row][col] = value;
     }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.board {
            for val in row {
                write!(f, "{: >4}", val)?; // Format each tile value with padding
            }
            writeln!(f)?;
        }
        Ok(())
    }
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_grid() {
        let mut grid = Grid::new();
        grid.set(0, 0, 20);
        assert_eq!(grid.get(0, 0), 20);
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
