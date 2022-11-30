use std::{collections::HashSet, fs};

const STEPS: u32 = 100;

type Point = (usize, usize);

fn step(point: Point, grid: &mut Vec<Vec<u32>>, flashed: &mut HashSet<Point>) {
    if flashed.contains(&point) {
        return;
    }
    if grid[point.0][point.1] >= 9 {
        flash(point, grid, flashed);
    } else {
        grid[point.0][point.1] += 1;
    }
}

fn flash(point: Point, grid: &mut Vec<Vec<u32>>, flashed: &mut HashSet<Point>) {
    if !flashed.insert(point) {
        return;
    }
    grid[point.0][point.1] = 0;

    let directions: &[(i32, i32); 8] = &[
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    for direction in directions {
        if point.0 <= 0 && direction.0.is_negative() {
            continue;
        }
        if point.1 <= 0 && direction.1.is_negative() {
            continue;
        }
        if point.0 >= grid.len() - 1 && direction.0.is_positive() {
            continue;
        }
        if point.1 >= grid.len() - 1 && direction.1.is_positive() {
            continue;
        }
        let point = (
            (point.0 as i32 + direction.0) as usize,
            (point.1 as i32 + direction.1) as usize,
        );
        step(point, grid, flashed);
    }
}

fn parse_grid(file: &str) -> Vec<Vec<u32>> {
    file.trim()
        .split('\n')
        .map(|line| {
            line.trim()
                .chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut grid = parse_grid(file.as_str());
    let mut total_flashes = 0;
    for _ in 0..STEPS {
        let mut flashed: HashSet<Point> = HashSet::new();
        for i in 0..grid.len() {
            for j in 0..grid.len() {
                step((i, j), &mut grid, &mut flashed);
            }
        }
        total_flashes += flashed.len();
    }

    println!("Total flashes: {}", total_flashes);

    let mut grid = parse_grid(file.as_str());
    let mut generation: u64 = 0;
    loop {
        generation += 1;
        let mut flashed: HashSet<Point> = HashSet::new();
        for i in 0..grid.len() {
            for j in 0..grid.len() {
                step((i, j), &mut grid, &mut flashed);
            }
        }
        if flashed.len() >= grid.len() * grid[0].len() {
            println!("Concurrent flash at generation {}", generation);
            break;
        }
    }
}
