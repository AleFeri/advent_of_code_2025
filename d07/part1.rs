fn main() {
    let mut input = read_input();
    let mut beams: Vec<char> = vec![];
    for i in 0..input[0].len() {
        beams.push(if input[0][i] == 'S' { '|' } else { '.' });
    }

    print_beams(&beams);

    input.remove(0); // remove the starting row

    let mut result = 0;
    for i in 0..input.len() {
        if !input[i].contains(&'^') {
            continue;
        }
        for j in 0..input[i].len() {
            if beams[j] == '|' && input[i][j] == '^' {
                beams[j - 1] = '|';
                beams[j + 1] = '|';
                beams[j] = '.';
                result += 1;
            }
        }
        print_beams(&beams);
    }
    println!("Result: {}", result);
}

fn print_beams(beams: &Vec<char>) {
    for beam in beams.iter() {
        print!("{}", beam);
    }
    println!();
}

fn read_input() -> Vec<Vec<char>> {
    std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}
