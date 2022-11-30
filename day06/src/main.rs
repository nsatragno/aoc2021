use std::fs;

const GENERATIONS: u32 = 256;

fn main() {
    let input: Vec<usize> = fs::read_to_string("input.txt")
        .expect("Could not open file")
        .trim()
        .split(',')
        .map(|number| number.parse().expect("Could not parse number"))
        .collect();

    let school: &mut [u64; 9] = &mut [0, 0, 0, 0, 0, 0, 0, 0, 0];
    for fish in input {
        school[fish] += 1;
    }

    for _ in 0..GENERATIONS {
        // Store the fish who are going to be spawning new fish.
        let spawning = school[0];

        // Age all the fish by a day.
        for i in 1..school.len() {
            school[i - 1] = school[i]
        }

        // Re add the fish that are spawning.
        school[6] += spawning;

        // Spawn the new fish.
        school[8] = spawning;
    }

    let total_fish = (0..school.len()).fold(0, |sum, i| sum + school[i]);
    println!("The total number of fish is: {total_fish}");
}
