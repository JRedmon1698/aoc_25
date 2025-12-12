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
    let input = "
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +   
    ";

    let g = Matrix::from_str(input);

    println!("{}", create_expressions_from_matrix(g));
}

impl Matrix<String> {
    fn from_str(s: &str) -> Self {
        let mut cells = Vec::new();
        let mut row_count = 0;
        let mut col_count = 0;

        for (row_idx, line) in s.lines().filter(|l| !l.trim().is_empty()).enumerate() {
            let parts: Vec<String> = line.split_whitespace().map(|x| x.to_string()).collect();

            if row_idx == 0 {
                col_count = parts.len();
            }

            for (col_idx, value) in parts.into_iter().enumerate() {
                cells.push(PointCell {
                    row: row_idx,
                    col: col_idx,
                    value,
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

    pub fn iter_points(&self) -> impl Iterator<Item = &PointCell<String>> {
        self.cells.iter()
    }

    pub fn get_point(&self, row: usize, col: usize) -> Option<&PointCell<String>> {
        if row < self.rows && col < self.cols {
            Some(&self.cells[row * self.cols + col])
        } else {
            None
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

fn create_expressions_from_matrix(input: Matrix<String>) -> bool {
    let mut total: u128 = 0;

    for col in 0..input.cols {
        let mut temp: Vec<String> = Vec::new();
        let mut sub_total: u128 = 0;

        for row in 0..input.rows {
            temp.push(input.get_point(row, col).unwrap().value.clone());
        }
        match temp.last().unwrap().as_str() {
            "+" => {
                for i in 0..temp.len() - 1 {
                    sub_total += temp[i].parse::<u128>().unwrap();
                }
            }
            "*" => {
                sub_total += 1;
                for i in 0..temp.len() - 1 {
                    sub_total *= temp[i].parse::<u128>().unwrap();
                }
            }
            _ => (),
        }
        total += sub_total;
    }

    println!("{:?}", total);

    false
}
