use std::fs;

fn main() {
    // First problem.
    let input: Vec<u32> = fs::read_to_string("input.txt")
        .expect("Could not open file")
        .trim()
        .split('\n')
        .map(|line| line.parse().expect("Could not parse line"))
        .collect();
    let mut count = 0;
    for index in 1..input.len() {
        if input[index - 1] < input[index] {
            count += 1;
        }
    }
    println!("Solution to first problem: {count}");

    // Second problem.
    let mut count = 0;
    let mut previous_window = input[0] + input[1] + input[2];
    for index in 3..input.len() {
        let current_window = input[index - 2] + input[index - 1] + input[index];
        if previous_window < current_window {
            count += 1;
        }
        previous_window = current_window;
    }
    println!("Solution to second problem: {count}");
}