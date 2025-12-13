use std::collections::{HashMap, HashSet};
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
    println!("{}", m.count_timelines());
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

    //PART 2
    pub fn count_timelines(&self) -> u128 {
        // find the 'S'
        let start_col = self
            .iter_row(0)
            .find(|c| c.value == 'S')
            .expect("'S' not found")
            .col as i32;

        // column -> number of timelines
        let mut timelines: HashMap<i32, u128> = HashMap::new();
        timelines.insert(start_col, 1);

        for row in 1..self.rows {
            let mut next: HashMap<i32, u128> = HashMap::new();

            for (&col, &count) in timelines.iter() {
                if col < 0 || col >= self.cols as i32 {
                    continue;
                }

                let cell = self.get_point(row, col as usize).unwrap();

                if cell.value == '^' {
                    *next.entry(col - 1).or_insert(0) += count;
                    *next.entry(col + 1).or_insert(0) += count;
                } else {
                    *next.entry(col).or_insert(0) += count;
                }
            }

            timelines = next;

            if timelines.is_empty() {
                break;
            }
        }

        timelines.values().sum()
    }

    // PART 1
    pub fn get_all_splitter_location_counts(&self) -> u128 {
        let mut total_splits: u128 = 0;

        // find the 'S'
        let start_col = self
            .iter_row(0)
            .find(|c| c.value == 'S')
            .expect("'S' not found")
            .col as i32;

        // active beams represented by column indexes
        let mut active_beams: HashSet<i32> = HashSet::new();
        active_beams.insert(start_col);

        // process row by row (starting below S)
        for row in 1..self.rows {
            let mut next_beams: HashSet<i32> = HashSet::new();

            for &col in &active_beams {
                // guard out of bounds
                if col < 0 || col >= self.cols as i32 {
                    continue;
                }

                let cell = self.get_point(row, col as usize).unwrap();

                if cell.value == '^' {
                    // split occurs
                    total_splits += 1;
                    next_beams.insert(col - 1);
                    next_beams.insert(col + 1);
                } else {
                    // beam continues straight down
                    next_beams.insert(col);
                }
            }

            active_beams = next_beams;

            // no beams left → stop early
            if active_beams.is_empty() {
                break;
            }
        }

        total_splits
    }

    pub fn iter_row(&self, row: usize) -> impl Iterator<Item = &PointCell<char>> {
        let start = row * self.cols;
        let end = start + self.cols;
        self.cells[start..end].iter()
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = impl Iterator<Item = &PointCell<char>>> + '_ {
        (0..self.rows).map(move |row| self.iter_row(row))
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
