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
    let grid_raw = String::from(
        "
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
",
    );

    // let m = generate_matrix_from_string(grid_raw);
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
