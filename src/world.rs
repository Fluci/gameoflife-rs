pub type CellIndex = usize;

pub struct World {
    rows: usize,
    cols: usize,
    matrix: Vec<bool>,
}

impl World {
    pub fn new_desert(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            matrix: vec![false; rows * cols],
        }
    }

    pub fn get_rows(&self) -> usize {
        self.rows
    }

    pub fn get_cols(&self) -> usize {
        self.cols
    }

    pub fn get_cell(&self, row: CellIndex, col: CellIndex) -> bool {
        self.check_bounds(row, col);
        self.matrix[row as usize * self.cols + col as usize]
    }

    pub fn set_cell(&mut self, row: CellIndex, col: CellIndex, new_value: bool) {
        self.check_bounds(row, col);
        self.matrix[row as usize * self.cols + col as usize] = new_value;
    }

    pub fn get_alive_neighbors_count(&self, row_u: usize, col_u: usize) -> usize {
        let row = row_u as i64;
        let col = col_u as i64;
        return self.get_padded_cell(row - 1, col - 1) as usize
            + self.get_padded_cell(row - 1, col) as usize
            + self.get_padded_cell(row - 1, col + 1) as usize
            + self.get_padded_cell(row, col - 1) as usize
            + self.get_padded_cell(row, col + 1) as usize
            + self.get_padded_cell(row + 1, col - 1) as usize
            + self.get_padded_cell(row + 1, col) as usize
            + self.get_padded_cell(row + 1, col + 1) as usize;
    }
}

impl World {
    fn get_padded_cell(&self, row: i64, col: i64) -> bool {
        if row == -1 || row as usize == self.rows || col == -1 || col as usize == self.cols {
            return false;
        }
        return self.get_cell(row as usize, col as usize);
    }

    fn check_bounds(&self, row: CellIndex, col: CellIndex) {
        if row as usize >= self.rows || col as usize >= self.cols {
            panic!(
                "Out of bounds: ({:}, {:}) for matrix of size {:}x{:}",
                row, col, self.rows, self.cols
            );
        }
    }
}

/// Debug, testing methods
impl World {
    pub fn all_dead(&self) -> bool {
        !self.matrix.iter().any(|c| *c == true)
    }
}

impl std::fmt::Debug for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //f.write_str("hello you!");
        let horizontal_bar = "+".to_string()
            + &std::iter::repeat("-")
                .take(self.get_cols())
                .collect::<String>()
            + "--+\n";
        f.write_str(&horizontal_bar).unwrap();
        for row in 0..self.get_rows() {
            f.write_str("| ").unwrap();
            for col in 0..self.get_cols() {
                if self.get_cell(row, col) {
                    f.write_str("#").unwrap()
                } else {
                    f.write_str(" ").unwrap();
                }
            }
            f.write_str(" |").unwrap();
            f.write_str("\n").unwrap();
        }
        f.write_str(&horizontal_bar).unwrap();
        f.write_str("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let world = World::new_desert(10, 8);
        assert_eq!(world.get_rows(), 10);
        assert_eq!(world.get_cols(), 8);
        assert_eq!(world.get_cell(0, 0), false);
        assert_eq!(world.get_cell(0, 7), false);
        assert_eq!(world.get_cell(9, 0), false);
        assert_eq!(world.get_cell(9, 7), false);
    }

    #[test]
    #[should_panic]
    fn test_out_of_bounds_1() {
        let world = World::new_desert(10, 8);
        world.get_cell(10, 0);
    }

    #[test]
    #[should_panic]
    fn test_out_of_bounds_2() {
        let world = World::new_desert(10, 8);
        world.get_cell(0, 8);
    }

    #[test]
    #[should_panic]
    fn test_out_of_bounds_3() {
        let world = World::new_desert(10, 8);
        world.get_cell(10, 8);
    }

    #[test]
    fn test_setter() {
        let mut world = World::new_desert(17, 19);
        assert_eq!(world.get_cell(15, 13), false);
        world.set_cell(15, 13, true);
        assert_eq!(world.get_cell(15, 13), true);
    }
}
