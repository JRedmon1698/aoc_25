#[derive(Debug, Clone)]
struct Turn {
    direction: char,
    number: u128,
}

fn main() {
    let doc = String::from(
        "
        R28
        L500
    ",
    );

    let parsed_doc = build_turns(doc);
    let number_of_zeroes = iterate_turns(parsed_doc);

    println!("{number_of_zeroes}");
}

fn build_turns(doc: String) -> Vec<Turn> {
    let mut result: Vec<Turn> = Vec::new();

    let split_doc: Vec<&str> = doc.split_whitespace().collect();

    for turn_str in split_doc {
        let mut chars = turn_str.chars();
        let direction = chars.next().unwrap();
        let number: u128 = chars.as_str().parse().unwrap();
        result.push(Turn { direction, number });
    }

    result
}

fn iterate_turns(turns: Vec<Turn>) -> u128 {
    let mut starting_pos = 50;
    let mut number_of_zeroes = 0;

    if starting_pos == 0 {
        number_of_zeroes += 1;
    }

    for turn in turns {
        starting_pos = record_position(starting_pos, turn);

        if starting_pos == 0 {
            number_of_zeroes += 1;
        }
    }

    number_of_zeroes
}

fn record_position(mut starting_pos: u128, turn: Turn) -> u128 {
    const MAX_POSITION: u128 = 100;

    match turn.direction {
        'L' => {
            let effective_turn = turn.number % MAX_POSITION;
            starting_pos = (starting_pos + MAX_POSITION - effective_turn) % MAX_POSITION;
        }
        'R' => {
            starting_pos = (starting_pos + turn.number) % MAX_POSITION;
        }
        _ => (),
    }

    starting_pos
}
