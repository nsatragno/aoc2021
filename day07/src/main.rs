use std::fs;

fn main() {
    let crabs: Vec<u32> = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect();

    let max = crabs.iter().copied().max().unwrap();
    let min = crabs.iter().copied().min().unwrap();

    // Part 1:
    let mut min_fuel = u32::MAX;
    let mut min_position = u32::MAX;
    for position in min..=max {
        let fuel = crabs
            .iter()
            .copied()
            .fold(0, |fuel, crab| fuel + position.abs_diff(crab));
        if fuel < min_fuel {
            min_fuel = fuel;
            min_position = position;
        }
    }

    println!("Part 1:");
    println!("The total position is {min_position}");
    println!("The total fuel consumption is {min_fuel}");

    // Part 2:
    let mut min_fuel = u32::MAX;
    let mut min_position = u32::MAX;
    for position in min..=max {
        let fuel = crabs.iter().copied().fold(0, |fuel, crab| {
            fuel + (0..=position.abs_diff(crab))
                .into_iter()
                .reduce(|a, b| a + b).unwrap()
        });
        if fuel < min_fuel {
            min_fuel = fuel;
            min_position = position;
        }
    }
    println!("Part 2:");
    println!("The total position is {min_position}");
    println!("The total fuel consumption is {min_fuel}");
}
