fn main() {
    let input = String::from("");

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
            // since we're only interested in codes that consist solely of repeated pairs
            // we can ignore odd-length numbers
            if i.to_string().len() % 2 == 1 {
                continue;
            }

            if code_is_bad(i) {
                total_inv_codes += i;
            }
        }
    }

    total_inv_codes
}

fn code_is_bad(code: u128) -> bool {
    let code_to_string = code.to_string();
    let (first_half, second_half) = code_to_string.split_at(code_to_string.len() / 2);

    first_half == second_half
}
