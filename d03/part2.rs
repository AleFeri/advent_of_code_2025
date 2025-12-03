use std::fs::read_to_string;

fn main() {
    let file_path = "input.txt";

    let mut result = 0;
    for line in read_lines(file_path) {
        let chars: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let len = chars.len();

        let mut last_max_tuple = (0, 0);
        for i in 0..12 {
            let prev_pos = last_max_tuple.1;
            last_max_tuple = find_max_with_pos(&chars[last_max_tuple.1..len-(11-i)]);
            last_max_tuple.1 += prev_pos + 1;
            result += last_max_tuple.0 as u64 * calc_pow(10, 11-i as u64);
        }
    }

    println!("{}", result);
}

fn calc_pow(base: u64, exponent: u64) -> u64 {
    if exponent == 0 {
        1
    } else {
        base * calc_pow(base, exponent - 1)
    }
}

fn find_max_with_pos(list: &[u32]) -> (u32, usize) {
    let mut pos = 0;
    let mut max = list[0];

    for (i, &num) in list.iter().enumerate() {
        if num > max {
            max = num;
            pos = i;
        }
    }

    (max, pos)
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename).unwrap().lines().map(String::from).collect()
}
