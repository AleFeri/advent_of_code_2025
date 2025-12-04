use std::vec::Vec;

fn main() {
    let file_path = "input.txt";
    let directions = [
        (-1, -1), (0, -1), (1, -1),
        (-1, 0),           (1, 0),
        (-1, 1),  (0, 1),  (1, 1)
    ];

    let mut matrix = matrix_from_file(file_path);
    let mut result = 0;
    loop {
        let mut removed = false;
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j] == '.' {
                    continue;
                }
                if directions.iter().filter(|&(dx, dy)| {
                    let new_x = i as i32 + dx;
                    let new_y = j as i32 + dy;
                    new_x >= 0 && new_y >= 0 && new_x < matrix.len() as i32 && new_y < matrix[new_x as usize].len() as i32 && (matrix[new_x as usize][new_y as usize] == '@' || matrix[new_x as usize][new_y as usize] == 'X')
                }).count() >= 4 {
                    continue;
                }
                matrix[i][j] = 'X';
                result += 1;
                removed = true;
            }
        }
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j] == 'X' {
                    matrix[i][j] = '.';
                }
            }
        }
        println!();
        for line in &matrix {
            println!("{}", line.iter().collect::<String>());
        }
        if !removed {
            break;
        }
    }
    println!("Result: {}", result);
}

fn matrix_from_file(file_path: &str) -> Vec<Vec<char>> {
    let contents = std::fs::read_to_string(file_path).unwrap();
    contents.lines().map(|line| line.chars().collect()).collect()
}
