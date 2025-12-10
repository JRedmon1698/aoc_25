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
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    let m = Matrix::from_str(grid_raw);
    println!("{}", m);
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

// fn generate_matrix_from_string(raw: String) -> Vec<Vec<char>> {
//     raw.split_whitespace()
//         .map(|l| l.chars().collect())
//         .collect::<Vec<Vec<char>>>()
// }

// fn print_matrix<T: std::fmt::Display>(v: &Vec<Vec<T>>) {
//     println!("[");
//     for row in v {
//         print!("   [");
//         for (i, item) in row.iter().enumerate() {
//             if i > 0 {
//                 print!(", ");
//             }
//             print!("{}", item);
//         }
//         println!("],");
//     }
//     println!("]");
// }
