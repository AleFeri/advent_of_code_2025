use std::collections::HashMap;
use std::fs::read_to_string;
use petgraph::prelude::DiGraphMap;

fn main() {
    let file_name = "input.txt";
    let mut arcs: Vec<(String, String)> = Vec::new();
    for line in read_from_file(file_name) {
        let split = line.split(':').collect::<Vec<&str>>();
        let main_node = split[0].to_string();
        let child_nodes = split[1].split(' ').filter(|&x| !x.is_empty());
        for child_node in child_nodes {
            arcs.push((main_node.clone(), child_node.to_string()));
        }
    }

    let g: DiGraphMap<&str, ()> = arcs
        .iter()
        .map(|(a, b)| (a.as_str(), b.as_str()))
        .collect();

    let mut path_map: HashMap<(&str, bool, bool), u64> =  HashMap::new();
    println!("Result: {}", count_paths(&g, &"svr", &"out", &mut path_map, false, false));
}

fn count_paths<'a>(
    g: &'a DiGraphMap<&'a str, ()>,
    current: &'a str,
    target: &'a str,
    path_map: &mut HashMap<(&'a str, bool, bool), u64>,
    seen_dac: bool,
    seen_fft: bool,
) -> u64 {
    let seen_dac = seen_dac || current == "dac";
    let seen_fft = seen_fft || current == "fft";

    if current == target {
        return if seen_dac && seen_fft { 1 } else { 0 };
    }

    let cache_key = (current, seen_dac, seen_fft);
    if let Some(&cached) = path_map.get(&cache_key) {
        return cached;
    }

    let neighbors: Vec<&'a str> = g.neighbors(current).collect();
    let mut count = 0;
    for neighbor in neighbors {
        count += count_paths(g, neighbor, target, path_map, seen_dac, seen_fft);
    }
    path_map.insert(cache_key, count);
    count
}

fn read_from_file(filename: &str) -> Vec<String> {
    read_to_string(filename).unwrap().lines().map(String::from).collect()
}
