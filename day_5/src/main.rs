use std::cmp::max;
use std::collections::HashSet;

fn main() {
    let input = "
";

    let parsed = parse_list(input);
    let (ranges, fresh_ids) = (parsed[0].clone(), parsed[1].clone());
    let num_fresh = get_enumerated_fresh_ids(ranges);

    println!("{:?}", num_fresh);
}

// build an array of 2 vectors to hold the fresh ingreds ids and available ids
fn parse_list(ingreds: &str) -> [Vec<&str>; 2] {
    let blocks: Vec<&str> = ingreds.trim().split("\n\n").collect();

    let fresh_ranges: Vec<&str> = blocks[0].split_whitespace().collect();
    let available_ids: Vec<&str> = blocks[1].split_whitespace().collect();

    [fresh_ranges, available_ids]
}

fn get_enumerated_fresh_ids(fresh_ranges: Vec<&str>) -> u128 {
    let mut fresh_count: u128 = 0;

    let mut ranges: Vec<(u128, u128)> = fresh_ranges
        .into_iter()
        .map(|r| {
            let parts: Vec<u128> = r.split('-').map(|x| x.parse().unwrap()).collect();
            (parts[0], parts[1])
        })
        .collect();

    // sort ranges by start num
    ranges.sort_by_key(|r| r.0);

    // create vector for consolidated ranges
    let mut consolidated_ranges: Vec<(u128, u128)> = Vec::new();

    for (range_start, range_end) in ranges {
        if let Some(last) = consolidated_ranges.last_mut() {
            // if curr range start <= last inserted range end in consolidated
            if range_start <= last.1 {
                // update range in in consolidated - last.end = max(last.end, current.end)
                last.1 = last.1.max(range_end);
            } else {
                // otherwise push range to consolidated
                consolidated_ranges.push((range_start, range_end));
            }
        // if consolidated list is empty, push range
        } else {
            consolidated_ranges.push((range_start, range_end));
        }
    }

    println!("consolidated ranges: {:?}", consolidated_ranges);

    for range in consolidated_ranges {
        // count of numbers is inclusive
        fresh_count += range.1 - range.0 + 1;
    }

    fresh_count
}

fn determine_num_fresh(fresh_ranges: Vec<&str>, avail_ids: Vec<&str>) -> u128 {
    let mut fresh_set: HashSet<u128> = HashSet::new();

    let parsed_ids: Vec<u128> = avail_ids
        .into_iter()
        .map(|s| s.parse::<u128>().unwrap())
        .collect();

    let ranges: Vec<(u128, u128)> = fresh_ranges
        .into_iter()
        .map(|r| {
            let parts: Vec<u128> = r.split('-').map(|x| x.parse().unwrap()).collect();
            (parts[0], parts[1])
        })
        .collect();

    for id in parsed_ids {
        for (start, end) in &ranges {
            if id >= *start && id <= *end {
                fresh_set.insert(id);
                break;
            }
        }
    }

    fresh_set.len() as u128
}
