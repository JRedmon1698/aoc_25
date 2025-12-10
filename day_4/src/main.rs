use std::fmt;

#[derive(Debug, Clone)]
pub struct PointCell<T> {
    pub row: usize,
    pub col: usize,
    pub value: T,
}

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    cells: Vec<PointCell<T>>,
}

fn main() {
    let grid_raw = "
    
";

    let mut m = Matrix::from_str(grid_raw);
    // println!("{}", m);

    // let empty_neighbors = m.points_with_sparse_neighbors();
    // println!("number of rolls: {}", empty_neighbors.len());

    let total_removed = m.remove_all_accessible();
    println!("{}", total_removed);
}

impl Matrix<char> {
    fn from_str(s: &str) -> Self {
        let mut cells = Vec::new();
        let mut row_count = 0;
        let mut col_count = 0;

        for (row_idx, line) in s.lines().filter(|l| !l.trim().is_empty()).enumerate() {
            let line = line.trim_end();
            if row_idx == 0 {
                col_count = line.chars().count();
            }

            for (col_idx, ch) in line.chars().enumerate() {
                cells.push(PointCell {
                    row: row_idx,
                    col: col_idx,
                    value: ch,
                });
            }
            row_count += 1;
        }

        Self {
            rows: row_count,
            cols: col_count,
            cells,
        }
    }

    pub fn iter_points(&self) -> impl Iterator<Item = &PointCell<char>> {
        self.cells.iter()
    }

    pub fn get_point(&self, row: usize, col: usize) -> Option<&PointCell<char>> {
        if row < self.rows && col < self.cols {
            Some(&self.cells[row * self.cols + col])
        } else {
            None
        }
    }

    pub fn points_with_sparse_neighbors(&self) -> Vec<&PointCell<char>> {
        let mut result = Vec::new();

        for point in self.iter_points() {
            if point.value == '.' {
                continue;
            }

            let neighbors = self.check_neighbors(point);

            let non_empty_count = neighbors
                .iter()
                .filter(|(_, _, is_empty)| !is_empty)
                .count();

            if non_empty_count < 4 {
                result.push(point);
            }
        }

        result
    }

    pub fn check_neighbors(&self, target_point: &PointCell<char>) -> Vec<(usize, usize, bool)> {
        // coordinates for all a point's possible neighbors
        let deltas = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let mut neighbors = Vec::new();

        for (delta_row, delta_column) in deltas {
            // get neighbor coordinates
            let neighbor_row = target_point.row as isize + delta_row;
            let neighbor_column = target_point.col as isize + delta_column;

            // protect against out of bounds
            if neighbor_row >= 0
                && neighbor_row < self.rows as isize
                && neighbor_column >= 0
                && neighbor_column < self.cols as isize
            {
                let neighbor =
                    &self.cells[(neighbor_row as usize) * self.cols + (neighbor_column as usize)];
                let is_empty = neighbor.value == '.';
                neighbors.push((neighbor.row, neighbor.col, is_empty));
            }
        }

        neighbors
    }

    pub fn remove_all_accessible(&mut self) -> usize {
        let mut total_removed = 0;

        loop {
            let mut to_remove = Vec::new();

            for point in self.iter_points() {
                if point.value != '@' {
                    continue;
                }

                let neighbors = self.check_neighbors(point);

                let non_empty_count = neighbors
                    .iter()
                    .filter(|(_, _, is_empty)| !is_empty)
                    .count();

                if non_empty_count < 4 {
                    to_remove.push((point.row, point.col));
                }
            }

            if to_remove.is_empty() {
                break;
            }

            total_removed += to_remove.len();

            // remove them (turn '@' into 'x')
            for (r, c) in to_remove {
                let idx = r * self.cols + c;
                self.cells[idx].value = '.';
            }
        }

        total_removed
    }

    pub fn is_empty(&self, target_point: &PointCell<char>) -> bool {
        target_point.value == '.'
    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[")?;
        for row in 0..self.rows {
            write!(f, "   [")?;
            for col in 0..self.cols {
                let cell = &self.cells[row * self.cols + col];
                if col > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", cell.value)?;
            }
            writeln!(f, "],")?;
        }
        writeln!(f, "]")
    }
}
