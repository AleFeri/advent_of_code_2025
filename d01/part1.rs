use std::fs::read_to_string;

fn main() {
    let file_path = "input.txt";

    let mut position = 50;
    let mut password = 0;
    for mut line in read_lines(file_path) {
        let add = line.remove(0) == 'R';

        let number: i32 = line.parse().unwrap();
        position = (position + (if add {1} else {-1})*number) % 100;
        if position < 0 {
            position = 100 + position;
        }
        if position == 0 {
            password += 1;
        }
    }
    println!("{}", password);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename).unwrap().lines().map(String::from).collect()
}
