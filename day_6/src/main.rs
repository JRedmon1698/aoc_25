use std::iter;

#[derive(Debug)]
struct ProblemBlock {
    rows: Vec<String>,
    operator: char,
}

impl ProblemBlock {
    fn solve(&self) -> u128 {
        // 1. Determine Alignment based on Operator
        // Rule derived from puzzle description:
        // '+' = Left Align (pad right with spaces)
        // '*' = Right Align (pad left with spaces)
        let max_width = self.rows.iter().map(|s| s.len()).max().unwrap_or(0);

        let aligned_rows: Vec<String> = self
            .rows
            .iter()
            .map(|s| {
                let padding = max_width - s.len();
                match self.operator {
                    '+' => format!("{}{}", s, " ".repeat(padding)), // Left Align
                    '*' => format!("{}{}", " ".repeat(padding), s), // Right Align
                    _ => s.to_string(),
                }
            })
            .collect();

        // 2. Parse columns vertically
        let mut numbers = Vec::new();
        for col in 0..max_width {
            let mut num_str = String::new();
            for row in &aligned_rows {
                let ch = row.chars().nth(col).unwrap_or(' ');
                if !ch.is_whitespace() {
                    num_str.push(ch);
                }
            }
            if let Ok(num) = num_str.parse::<u128>() {
                numbers.push(num);
            }
        }

        // 3. Calculate
        match self.operator {
            '+' => numbers.iter().sum(),
            '*' => numbers.iter().product(),
            _ => 0,
        }
    }
}

fn parse_input(input: &str) -> Vec<ProblemBlock> {
    let lines: Vec<&str> = input.lines().filter(|l| !l.trim().is_empty()).collect();

    if lines.is_empty() {
        return vec![];
    }

    // Split each line into tokens (numbers/operators)
    let grid: Vec<Vec<&str>> = lines
        .iter()
        .map(|line| line.split_whitespace().collect())
        .collect();

    let rows_count = grid.len();
    let problems_count = grid[0].len(); // Assuming rectangular grid of tokens

    let mut problems = Vec::new();

    for p_idx in 0..problems_count {
        let mut num_rows = Vec::new();
        let mut operator = '+';

        for r_idx in 0..rows_count {
            let token = grid[r_idx].get(p_idx).unwrap_or(&"");
            // If it's the last row, it's the operator
            if r_idx == rows_count - 1 {
                operator = token.chars().next().unwrap_or('+');
            } else {
                num_rows.push(token.to_string());
            }
        }
        problems.push(ProblemBlock {
            rows: num_rows,
            operator,
        });
    }

    problems
}

fn main() {
    // We use the raw values. The logic handles the alignment,
    // so we don't need to worry about the spacing in this string.
    let input = "
    
";

    let problems = parse_input(input);
    let mut grand_total: u128 = 0;

    for (i, p) in problems.iter().enumerate() {
        let answer = p.solve();
        println!(
            "Problem {}: {:?} using '{}' -> {}",
            i + 1,
            p.rows,
            p.operator,
            answer
        );
        grand_total += answer;
    }

    println!("----------------");
    println!("Grand Total: {}", grand_total);
}
