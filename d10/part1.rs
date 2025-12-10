fn main() {
    let input = read_input();
    let mut result = 0;
    for line in &input {
        let (target, buttons) = extract_info(line);
        println!("Target: {:?}", target);
        println!("Buttons: {:?}", buttons);

        let line_result = solve_line(target, buttons);
        println!("Line Result: {}", line_result);
        result += line_result;
    }
    println!("Result: {}", result);
}

fn extract_info(line: &str) -> (Vec<bool>, Vec<Vec<usize>>) {
    let target_start = line.find('[').unwrap() + 1;
    let target_end = line.find(']').unwrap();
    let target: Vec<bool> = line[target_start..target_end]
        .chars()
        .map(|c| c == '#')
        .collect();

    let buttons_section = &line[target_end + 2..line.find('{').unwrap_or(line.len()) - 1];
    let buttons = buttons_section
        .split_whitespace()
        .map(|b| {
            b[1..b.len() - 1]
                .to_string()
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
        .clone();

    (target, buttons)
}

fn solve_line(target: Vec<bool>, buttons: Vec<Vec<usize>>) -> u32 {
    let mut min_presses = u32::MAX;

    // Iterate over all possible button combinations (Note for myself in the future)
    for mask in 0..(1u64 << buttons.len()) {
        let mut lights = vec![false; target.len()];
        let mut presses = 0;
        for (i, button) in buttons.iter().enumerate() {
            // check the mask (press or not)
            if (mask >> i) & 1 == 1 {
                presses += 1;
                // toggle the lights in the button positions
                for &light_idx in button {
                    lights[light_idx] ^= true;
                }
            }
        }
        if lights == target {
            min_presses = min_presses.min(presses);
        }
    }

    min_presses
}

fn read_input() -> Vec<String> {
    std::fs::read_to_string("input.txt")
        .expect("Failed to read input file")
        .lines()
        .map(|line| line.to_string())
        .collect()
}
