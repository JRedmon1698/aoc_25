/*
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124

find the invalid IDs by looking for any ID which is made only of some sequence of digits repeated twice.
So, 55 (5 twice), 6464 (64 twice), and 123123 (123 twice) would all be invalid IDs.

None of the numbers have leading zeroes; 0101 isn't an ID at all. (101 is a valid ID that you would ignore.)

11-22 has two invalid IDs, 11 and 22.
95-115 has one invalid ID, 99.
998-1012 has one invalid ID, 1010.
1188511880-1188511890 has one invalid ID, 1188511885.
222220-222224 has one invalid ID, 222222.
1698522-1698528 contains no invalid IDs.
446443-446449 has one invalid ID, 446446.
38593856-38593862 has one invalid ID, 38593859.
The rest of the ranges contain no invalid IDs.

Adding up all the invalid IDs in this example produces 1227775554.
*/

fn main() {
    let input = String::from(
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
    );

    // println!("{:?}", parse_codes(input));
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

fn find_invalid_codes_total(codes: Vec<[u128; 2]>) -> u32 {
    let mut total_inv_codes: u32 = 0;

    for code in codes {
        for i in code[0]..=code[1] {
            if i % 2 == 1 {
                continue;
            }

            // find bad codes here
        }
    }

    total_inv_codes
}
