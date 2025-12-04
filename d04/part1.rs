use std::vec::Vec;

fn main() {
    let file_path = "input.txt";
    let matrix = matrix_from_file(file_path);
    let mut result = 0;
    for (i, row) in matrix.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if cell == &'.' {
                print!(".");
                continue;
            }
            if !check_cell(&matrix, i, j) {
                print!("@");
                continue;
            }
            print!("X");
            result += 1;
        }
        println!();
    }
    println!("Result: {}", result);
}

fn check_cell(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let directions = [
        (-1, -1), (0, -1), (1, -1),
        (-1, 0),           (1, 0),
        (-1, 1),  (0, 1),  (1, 1)
    ];
    directions.iter().filter(|&(dx, dy)| {
        let new_x = x as i32 + dx;
        let new_y = y as i32 + dy;
        new_x >= 0 && new_y >= 0 && new_x < matrix.len() as i32 && new_y < matrix[new_x as usize].len() as i32 && matrix[new_x as usize][new_y as usize] == '@'
    }).count() < 4
}

fn matrix_from_file(file_path: &str) -> Vec<Vec<char>> {
    let contents = std::fs::read_to_string(file_path).unwrap();
    contents.lines().map(|line| line.chars().collect()).collect()
}
