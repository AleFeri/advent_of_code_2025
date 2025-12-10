fn main() {
    let input = read_input();
    let mut biggest_area: i128 = 0;
    for i in 0..input.len() {
        for j in (i+1)..input.len() {
            let area = (input[i].0 - input[j].0 + 1).abs() as i128 * (input[i].1 - input[j].1 + 1).abs() as i128;
            if area > biggest_area {
                biggest_area = area;
            }
        }
    }
    println!("Result: {}", biggest_area);
}

fn read_input() -> Vec<(i32, i32)> {
    std::fs::read_to_string("input.txt").unwrap().lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect()
}
