use std::fs::read_to_string;

fn main () {
    let mut result = 0;
    let mut numbers: Vec<(i128, i128)> = Vec::new();
    for line in read_input() {
        if line.chars().collect::<String>().contains("-") {
            let map = line.split("-").map(|s| s.parse::<i128>().unwrap()).collect::<Vec<i128>>();
            numbers.push((map[0], map[1]));
        }
        else {
            let number = &line.parse::<i128>().unwrap();
            if numbers.iter().any(|(a, b)| a <= number && number <= b) {
                result += 1;
            }
        }
    }
    println!("Result: {}", result);
}

fn read_input() -> Vec<String> {
    read_to_string("input.txt").expect("Failed to read input file").lines().map(String::from).filter(|line| !line.is_empty()).collect()
}
