use std::collections::HashSet;
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

    let mut visited = HashSet::new();
    println!("Result: {}", count_paths(&g, &"you", &"out", &mut visited));
}

fn count_paths<'a>(
    g: &'a DiGraphMap<&'a str, ()>,
    current: &'a str,
    target: &'a str,
    visited_nodes: &mut HashSet<&'a str>
) -> u32 {
    if current == target {
        return 1;
    }

    visited_nodes.insert(current);

    let mut count = 0;
    for neighbor in g.neighbors(current) {
        if !visited_nodes.contains(neighbor) {
            count += count_paths(g, neighbor, target, visited_nodes);
        }
    }

    visited_nodes.remove(current);
    count
}

fn read_from_file(filename: &str) -> Vec<String> {
    read_to_string(filename).unwrap().lines().map(String::from).collect()
}
