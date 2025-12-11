fn main() {
    let input = "
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    let parsed = parse_list(input);
    println!("{:?}", parsed);
}

// build an array of 2 vectors to hold the fresh ingreds ids and available ids
fn parse_list(ingreds: &str) -> [Vec<&str>; 2] {
    let blocks: Vec<&str> = ingreds.trim().split("\n\n").collect();

    let fresh_ids: Vec<&str> = blocks[0].split_whitespace().collect();
    let available_ids: Vec<&str> = blocks[1].split_whitespace().collect();

    [fresh_ids, available_ids]
}
