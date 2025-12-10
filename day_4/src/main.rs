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

    let m = generate_matrix_from_string(grid_raw);
    print_matrix(&m);
}

// fn check_adjacent_neighbors<T>(point: Point<T>, grid: &Vec<Vec<T>>) -> bool {
//     // let

//     false
// }

fn generate_matrix_from_string(raw: String) -> Vec<Vec<char>> {
    raw.split_whitespace()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>()
}

fn print_matrix<T: std::fmt::Display>(v: &Vec<Vec<T>>) {
    println!("[");
    for row in v {
        print!("   [");
        for (i, item) in row.iter().enumerate() {
            if i > 0 {
                print!(", ");
            }
            print!("{}", item);
        }
        println!("],");
    }
    println!("]");
}
