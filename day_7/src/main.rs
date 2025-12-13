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
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    let mut m = Matrix::from_str(grid_raw);
    println!("{}", m);
    println!("{}", m.get_all_splitter_location_counts());
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

    pub fn get_all_splitter_location_counts(&self) -> u128 {
        let splitter_char: char = '^';
        let mut total_split_count: u128 = 0;
        let mut splitter_char_indexes: Vec<Vec<u128>> = Vec::new();

        for (row_idx, row) in self.iter_rows().enumerate() {
            let mut curr_row_splitter_indexes: Vec<u128> = Vec::new();
            for cell in row {
                if cell.value == splitter_char {
                    curr_row_splitter_indexes.push(cell.col as u128);
                }
            }

            splitter_char_indexes.push(curr_row_splitter_indexes);
        }

        // a vector to track where splits SHOULD happen
        // initialize to the index in the first row - index of 'S'
        let mut line_to_be_split_at_indexes: &Vec<u128> = &self
            .iter_row(0)
            .find(|cell| cell.value == 'S')
            .map(|cell| vec![cell.col as u128])
            .expect("'S' not found in first row");

        for row_splitter_indexes in &splitter_char_indexes {
            // skip the first row [that contains the 'S']
            if row_splitter_indexes != splitter_char_indexes.iter().nth(0).unwrap() {
                total_split_count += row_splitter_indexes
                    .iter()
                    .filter(|x| {
                        line_to_be_split_at_indexes.contains(&(*x + 1))
                            || line_to_be_split_at_indexes.contains(&(*x - 1))
                    })
                    .count() as u128;

                line_to_be_split_at_indexes = row_splitter_indexes;
            }
            // if row_splitter_indexes.iter().filter(|x| line_to_be_split_at_indexes.contains(x)).count() {}
        }

        total_split_count
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
