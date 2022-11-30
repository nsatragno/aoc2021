use std::{collections::HashMap, fs};

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from(string: &str) -> Point {
        let mut pieces = string.split(',');
        Point {
            x: pieces.next().unwrap().parse().unwrap(),
            y: pieces.next().unwrap().parse().unwrap(),
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Line {
    a: Point,
    b: Point,
}

impl Line {
    fn from(string: &str) -> Line {
        let mut pieces = string.trim().split(' ');
        let a = Point::from(pieces.next().unwrap());
        pieces.next().unwrap(); // Discard the arrow.
        let b = Point::from(pieces.next().unwrap());
        Line { a, b }
    }

    fn points(&self) -> Vec<Point> {
        let dx: i32 = if self.a.x < self.b.x {
            1
        } else if self.a.x == self.b.x {
            0
        } else {
            -1
        };
        let dy: i32 = if self.a.y < self.b.y {
            1
        } else if self.a.y == self.b.y {
            0
        } else {
            -1
        };
        let mut x: i32 = self.a.x;
        let mut y: i32 = self.a.y;
        let mut points: Vec<Point> = Vec::new();
        while x != self.b.x || y != self.b.y {
            points.push(Point { x, y });
            x += dx;
            y += dy;
        }
        points.push(self.b.clone());
        points
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.trim().split('\n').map(Line::from);
    let mut map: HashMap<Point, bool> = HashMap::new();
    for line in lines {
        for point in line.points() {
            map.entry(point)
                .and_modify(|slot| *slot = true)
                .or_insert(false);
        }
    }
    let result = map.values().filter(|val| **val).count();
    println!("The result is {}", result);
}
