use std::fs::read_to_string;
use std::collections::HashSet;

fn main() {
    let file_path = "input.txt";

    let input = read_line(file_path);

    println!("Input: {}", input);
    let mut result: i128 = 0;
    for id_range in input.split(',') {
        let id1: i128 = id_range.split('-').next().unwrap().parse().unwrap();
        let id2: i128 = id_range.split('-').nth(1).unwrap().parse().unwrap();

        for id in id1..=id2 {
            if has_repetitions(id) {
                result += id;
            }
        }
    }
    println!("Result: {}", result);
}

fn has_repetitions(id: i128) -> bool {
    let sid: &str = &id.to_string();
    let len: usize = sid.len();
    for partition in 2..=len {
        let mut found = HashSet::new();
        if len % partition != 0 {
            continue;
        }
        let step = len/partition;
        for pos in (0..=(len-step)).step_by(step) {
            found.insert(&sid[pos..pos+step]);
        }
        if found.len() == 1 {
            return true;
        }
    }
    return false;
}

fn read_line(filename: &str) -> String {
    String::from(read_to_string(filename).unwrap().clone().strip_suffix('\n').unwrap())
}
