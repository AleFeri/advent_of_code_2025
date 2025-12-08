use std::collections::HashSet;

fn main() {
    let input = read_input("input.txt");

    let mut distances: Vec<(String, String, f64)> = Vec::new();
    let names: Vec<String> = input.iter().map(|line| create_name(line)).collect();

    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            distances.push((
                names[i].clone(),
                names[j].clone(),
                euclidean_distance(&input[i], &input[j]),
            ));
        }
    }

    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let mut groups: Vec<HashSet<String>> = input
        .iter()
        .map(|line| HashSet::from([create_name(line)]))
        .collect();

    let mut last_mul = 0;
    for i in 0..distances.len() {
        let (from, to, _) = &distances[i];

        let matching_indices: Vec<usize> = groups
            .iter()
            .enumerate()
            .filter(|(_, group)| group.contains(from) || group.contains(to))
            .map(|(idx, _)| idx)
            .collect();

        if matching_indices.len() == 1 {
            continue;
        }

        let second_group = groups[matching_indices[1]].clone();
        groups[matching_indices[0]].extend(second_group);
        last_mul = from.split("_").next().unwrap().parse::<i128>().unwrap()
            * to.split("_").next().unwrap().parse::<i128>().unwrap();
        groups.remove(matching_indices[1]);
    }

    groups.sort_by(|g1, g2| g2.len().cmp(&g1.len()));

    println!("Result: {}", last_mul);
}

fn create_name(node: &[i128]) -> String {
    format!("{}_{}_{}", node[0], node[1], node[2])
}

fn euclidean_distance(a: &[i128], b: &[i128]) -> f64 {
    (((a[0] - b[0]).pow(2) + (a[1] - b[1]).pow(2) + (a[2] - b[2]).pow(2)) as f64).sqrt()
}

fn read_input(filename: &str) -> Vec<Vec<i128>> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse().unwrap())
                .collect::<Vec<i128>>()
        })
        .collect()
}
