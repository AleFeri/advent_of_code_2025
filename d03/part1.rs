use std::fs::read_to_string;

fn main() {
    let file_path = "input.txt";

    let mut result = 0;
    for line in read_lines(file_path) {
        let chars: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let max1_tuple = find_max_with_pos(&chars.split_last().unwrap().1);
        let max2_tuple = find_max_with_pos(&chars[max1_tuple.1+1..]);

        result += max1_tuple.0*10 + max2_tuple.0;
    }

    println!("{}", result);
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
