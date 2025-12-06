fn main() {
    let mut input_str = read_file("input.txt");
    let operations = input_str[input_str.len() - 1].clone();
    input_str.pop();

    let max_line = input_str.iter().map(|row| row.len()).max().unwrap_or(0);

    let mut result = 0;
    let mut last_number = String::new();
    let mut last_op = ' ';
    for col in 0..max_line {
        if col < operations.len() && operations[col] != ' ' {
            last_op = operations[col];
        }
        let mut number: String = String::new();
        for row in 0..input_str.len() {
            if col < input_str[row].len() {
                number.push(input_str[row][col]);
            }
        }
        number = number.trim().to_string();
        if last_number.is_empty() {
            last_number = number;
            continue;
        }
        if number.is_empty() {
            result += last_number.parse::<i128>().unwrap();
            last_number.clear();
            last_op = ' ';
            continue;
        }
        if last_op == '+' {
            last_number = (last_number.parse::<i128>().unwrap() + number.parse::<i128>().unwrap())
                .to_string();
        } else if last_op == '*' {
            last_number = (last_number.parse::<i128>().unwrap() * number.parse::<i128>().unwrap())
                .to_string();
        }
    }
    if !last_number.is_empty() {
        result += last_number.parse::<i128>().unwrap();
    }
    println!("Result: {}", result);
}

fn read_file(filename: &str) -> Vec<Vec<char>> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}
