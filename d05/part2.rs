use std::fs::read_to_string;
use std::collections::HashSet;

fn main () {
    let mut numbers: HashSet<(i128, i128)> = HashSet::new();

    for line in read_input() {
        if !line.chars().collect::<String>().contains("-") {
            break;
        }
        let map = line.split("-").map(|s| s.parse::<i128>().unwrap()).collect::<Vec<i128>>();
        let new_range = (map[0], map[1]);

        if numbers.iter().find(|range| new_range.0 >= range.0 && new_range.1 <= range.1).cloned().is_some() {
            continue;
        }

        let to_merge = numbers.iter()
            .find(|range| {
                (new_range.0 < range.0 && new_range.1 >= range.0) ||
                (new_range.0 <= range.1 && new_range.1 > range.1)
            })
            .cloned();
        if to_merge.is_none() {
            numbers.insert(new_range);
            continue;
        }

        if let Some(range) = to_merge {
            numbers.remove(&range);
            numbers.insert((
                new_range.0.min(range.0),
                new_range.1.max(range.1)
            ));
        }
    }

    loop {
        let mut mergiable_ranges: Vec<(i128, i128)> = Vec::new();
        for range in numbers.iter().cloned() {
            let mergiable = find_mergiable(&range, &numbers);
            if !mergiable.is_empty() {
                mergiable.iter().for_each(|r| {
                    mergiable_ranges.push(r.clone());
                });
                break;
            }
        }

        if mergiable_ranges.is_empty() {
            break;
        }

        mergiable_ranges.iter().for_each(|r| {numbers.remove(&r);});
        let min = mergiable_ranges.iter().min_by_key(|r| r.0).unwrap().0;
        let max = mergiable_ranges.iter().max_by_key(|r| r.1).unwrap().1;
        numbers.insert((min, max));
    }

    let result = numbers.iter().map(|(start, end)| end - start + 1).sum::<i128>();
    println!("Result: {}", result);
}

fn find_mergiable(range: &(i128, i128), numbers: &HashSet<(i128, i128)>) -> Vec<(i128, i128)> {
    let mut mergiable: Vec<_> = numbers.iter().filter(|r| &range != r && (range.0 < r.0 && range.1 >= r.0) || (range.0 <= r.1 && range.1 > r.1)).cloned().collect();
    if mergiable.is_empty() {Vec::new()} else {
        mergiable.push(range.clone());
        mergiable
    }
}

fn read_input() -> Vec<String> {
    read_to_string("input.txt").expect("Failed to read input file").lines().map(String::from).filter(|line| !line.is_empty()).collect()
}
