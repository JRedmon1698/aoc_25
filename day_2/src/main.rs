fn main() {
    let input = String::from("");
    // println!("{:?}", get_divisors(&10));

    println!("{:?}", find_invalid_codes_total(parse_codes(input)));
}

fn parse_codes(input: String) -> Vec<[u128; 2]> {
    input
        .split(',')
        .map(|range| {
            let parts: Vec<u128> = range.split('-').map(|n| n.parse().unwrap()).collect();
            [parts[0], parts[1]]
        })
        .collect()
}

fn find_invalid_codes_total(codes: Vec<[u128; 2]>) -> u128 {
    let mut total_inv_codes: u128 = 0;

    for code in codes {
        for i in code[0]..=code[1] {
            // PART1 ONLY: since we're only interested in codes that consist solely of repeated pairs
            // we can ignore odd-length numbers
            // if i.to_string().len() % 2 == 1 {
            //     continue;
            // }

            if code_is_bad(i) {
                total_inv_codes += i;
            }
        }
    }

    total_inv_codes
}

// PART 2
fn code_is_bad(code: u128) -> bool {
    let code_string = code.to_string();
    let dividend: u32 = code_string.clone().len() as u32;
    let divisors: Vec<u32> = get_divisors(&dividend);

    // for each divisor n of the code's length
    // check up to &code_string[..n + 1]
    // figure out how many times that pattern is needed to complete string (code_string.len / n)
    // repeat the slice pattern that many times and compare to code_string
    let mut pattern = String::new();
    for divisor in divisors {
        // if divisor == dividend {
        //     continue; // skip full length
        // }
        let sub_str = &code_string[..divisor as usize];
        let repeat_num = dividend / divisor;
        pattern = sub_str.repeat(repeat_num as usize);

        if pattern == code_string {
            return true;
        }
    }

    false
}

fn get_divisors(n: &u32) -> Vec<u32> {
    let mut divisors = Vec::new();
    for i in 1..*n {
        if n % i == 0 {
            divisors.push(i);
        }
    }

    divisors
}

// PART 1
// fn code_is_bad(code: u128) -> bool {
//     let code_to_string = code.to_string();
//     let (first_half, second_half) = code_to_string.split_at(code_to_string.len() / 2);

//     first_half == second_half
// }
