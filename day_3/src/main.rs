fn main() {
    let input = String::from(
        "
",
    );

    println!("{:?}", calculate_joltage(parse_input(input)));
}

fn parse_input(input: String) -> Vec<String> {
    input
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

// PART 2
fn calculate_joltage(banks: Vec<String>) -> u128 {
    let mut joltage: u128 = 0;
    let joltage_length = 12;

    for bank in banks {
        let digits: Vec<u128> = bank
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u128)
            .collect();

        let mut stack: Vec<u128> = Vec::new();
        let mut to_remove: usize = digits.len() - joltage_length;

        for d in digits {
            // short circuiting: if any condition is false the rest are skipped
            while to_remove > 0 && !stack.is_empty() && stack.last().unwrap() < &d {
                stack.pop();
                to_remove -= 1;
            }

            stack.push(d);
        }

        // we're only interested in a number of length 12
        stack.truncate(joltage_length);

        // turn the stack [vector] of numbers into a single number
        // start with 0, and multiply acc by 10 and add the next number in the stack
        let value = stack.iter().fold(0, |acc, &d| acc * 10 + d);
        joltage += value;
    }

    joltage
}

// PART 1
// fn calculate_joltage(banks: Vec<String>) -> u128 {
//     let mut joltage: u128 = 0;

//     for bank in banks {
//         let digits: Vec<u128> = bank
//             .chars()
//             .map(|c| c.to_digit(10).unwrap() as u128)
//             .collect();

//         let mut biggest_joltage: u128 = 0;

//         for i in 0..digits.len() {
//             for j in i + 1..digits.len() {
//                 let value = digits[i] * 10 + digits[j];
//                 if value > biggest_joltage {
//                     biggest_joltage = value;
//                 }
//             }
//         }

//         joltage += biggest_joltage as u128;
//     }

//     joltage
// }
