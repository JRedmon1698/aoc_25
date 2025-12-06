#[derive(Debug, Clone)]
struct Turn {
    direction: char,
    number: u8,
}

fn main() {
    let doc = String::from(
        "
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82
    ",
    );

    let parsed_doc = build_turns(doc);

    // println!("{:?}", positions);
}

fn build_turns(doc: String) -> Vec<Turn> {
    let mut result: Vec<Turn> = Vec::new();

    let split_doc: Vec<&str> = doc.split_whitespace().collect();

    for turn_str in split_doc {
        let mut chars = turn_str.chars();
        let direction = chars.next().unwrap();
        let number: u8 = chars.as_str().parse().unwrap();
        result.push(Turn { direction, number });
    }

    result
}

fn iterate_turns(turns: Vec<Turn>) -> () {
    let mut starting_pos = 50;
    let mut positions = Vec::new();

    for turn in turns {
        record_positions(&mut starting_pos, turn, positions.clone());
    }
}

fn record_positions(starting_pos: &mut u8, turn: Turn, mut resting_positions: Vec<u8>) -> Vec<u8> {
    let mut resting_position: u8 = starting_pos.clone();

    match turn.direction {
        'L' => {
            if turn.number > *starting_pos {
                let difference = turn.number - *starting_pos;
                resting_position = 100 - difference;
                resting_positions.push(resting_position);
            }
        }
        'R' => resting_positions.push(*starting_pos + turn.number),
        _ => (),
    }

    resting_positions
}
