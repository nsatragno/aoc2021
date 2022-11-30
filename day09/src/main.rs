use std::{collections::HashSet, fs};

fn step(
    height: &u32,
    next_point: (usize, usize),
    map: &Vec<Vec<u32>>,
    solved: &mut HashSet<(usize, usize)>,
) -> u32 {
    let next = map[next_point.0][next_point.1];
    if next > *height && next < 9 && !solved.contains(&next_point) {
        find_basin(&next_point, map, solved)
    } else {
        0
    }
}

fn find_basin(
    point: &(usize, usize),
    map: &Vec<Vec<u32>>,
    solved: &mut HashSet<(usize, usize)>,
) -> u32 {
    let mut size = 1;
    solved.insert(point.clone());
    let current = map[point.0][point.1];
    if point.0 != 0 {
        size += step(&current, (point.0 - 1, point.1), map, solved);
    }
    if point.1 != 0 {
        size += step(&current, (point.0, point.1 - 1), map, solved);
    }
    if point.0 != map.len() - 1 {
        size += step(&current, (point.0 + 1, point.1), map, solved);
    }
    if point.1 != map[point.0].len() - 1 {
        size += step(&current, (point.0, point.1 + 1), map, solved);
    }
    size
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let input: Vec<Vec<u32>> = file
        .trim()
        .split('\n')
        .map(|line| {
            line.trim()
                .chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let mut low_points: Vec<(usize, usize)> = Vec::new();
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if (i == 0 || input[i - 1][j] > input[i][j])
                && (j == 0 || input[i][j - 1] > input[i][j])
                && (i == input.len() - 1 || input[i + 1][j] > input[i][j])
                && (j == input[i].len() - 1 || input[i][j + 1] > input[i][j])
            {
                low_points.push((i, j));
            }
        }
    }

    let risk_level_sum: u32 = low_points
        .iter()
        .map(|point| input[point.0][point.1] + 1)
        .sum();
    println!("The risk level sum is {}", risk_level_sum);

    let mut solved: HashSet<(usize, usize)> = HashSet::new();
    let mut largest: Vec<u32> = low_points
        .iter()
        .map(|point| find_basin(&point, &input, &mut solved))
        .collect();
    largest.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());

    println!(
        "The three largest are {}, {}, and {}",
        largest[0], largest[1], largest[2],
    );
    println!("The result is: {}", largest[0] * largest[1] * largest[2]);
}
