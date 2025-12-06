fn main() {
    let input = read_file("input.txt");
    let mut result = 0;
    for i in 0..input[0].len() {
        result += match input[4][i].parse::<char>().unwrap() {
            '+' => {
                input[0][i].parse::<i128>().unwrap()
                    + input[1][i].parse::<i128>().unwrap()
                    + input[2][i].parse::<i128>().unwrap()
                    + input[3][i].parse::<i128>().unwrap()
            }
            '*' => {
                input[0][i].parse::<i128>().unwrap()
                    * input[1][i].parse::<i128>().unwrap()
                    * input[2][i].parse::<i128>().unwrap()
                    * input[3][i].parse::<i128>().unwrap()
            }
            _ => panic!("Invalid operator"),
        };
    }
    println!("Result: {}", result);
}

fn read_file(filename: &str) -> Vec<Vec<String>> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.trim().split_whitespace().map(String::from).collect())
        .collect()
}
