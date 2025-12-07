use dashmap::DashMap;
use rayon::join;
use std::sync::Arc;

fn main() {
    let mut input = read_input();
    let beam_pos = input[0].iter().position(|x| *x == 'S').unwrap();

    input.remove(0); // remove the starting row

    let result = visit_cached(&input, beam_pos);
    println!("Result: {}", result);
}

fn visit_cached(input: &[Vec<char>], beam_pos: usize) -> u128 {
    let cache = Arc::new(DashMap::new());
    visit(input, beam_pos, 0, &cache)
}

fn visit(
    input: &[Vec<char>],
    beam_pos: usize,
    depth: usize,
    cache: &Arc<DashMap<(usize, usize), u128>>,
) -> u128 {
    let key = (depth, beam_pos);
    if input.is_empty() {
        cache.insert(key, 1);
        return 1;
    }
    if let Some(result) = cache.get(&key) {
        return *result;
    }
    let rest = &input[1..];
    let result = if input[0][beam_pos] == '^' {
        let (left, right) = join(
            || visit(&rest, beam_pos - 1, depth + 1, cache),
            || visit(&rest, beam_pos + 1, depth + 1, cache),
        );
        left + right
    } else {
        visit(&rest, beam_pos, depth + 1, cache)
    };

    cache.insert(key, result);
    result
}

fn read_input() -> Vec<Vec<char>> {
    std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}
