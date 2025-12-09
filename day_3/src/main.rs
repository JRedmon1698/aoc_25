fn main() {
    let input = String::from(
        "
",
    );

    // println!("{}", input);
    println!("{:?}", calculate_joltage(parse_input(input)));
}

fn parse_input(input: String) -> Vec<String> {
    input
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

fn calculate_joltage(banks: Vec<String>) -> u128 {
    let mut joltage: u128 = 0;

    for bank in banks {
        let digits: Vec<u128> = bank
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u128)
            .collect();

        let mut biggest_joltage: u128 = 0;

        for i in 0..digits.len() {
            for j in i + 1..digits.len() {
                let value = digits[i] * 10 + digits[j];
                if value > biggest_joltage {
                    biggest_joltage = value;
                }
            }
        }

        joltage += biggest_joltage as u128;
    }

    joltage
}
