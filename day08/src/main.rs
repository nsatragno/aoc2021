use std::{collections::HashSet, fmt, fs, str};

struct Problem {
    digits: [Digit; 10],
    output: [Digit; 4],
}

impl Problem {
    fn new(string: &str) -> Problem {
        let mut split = string.trim().split(' ');
        let digits = [
            Digit::new(split.next().unwrap()),
            Digit::new(split.next().unwrap()),
            Digit::new(split.next().unwrap()),
            Digit::new(split.next().unwrap()),
            Digit::new(split.next().unwrap()),
            Digit::new(split.next().unwrap()),
            Digit::new(split.next().unwrap()),
            Digit::new(split.next().unwrap()),
            Digit::new(split.next().unwrap()),
            Digit::new(split.next().unwrap()),
        ];
        split.next(); // Consume the |.
        let output = [
            Digit::new(split.next().unwrap()),
            Digit::new(split.next().unwrap()),
            Digit::new(split.next().unwrap()),
            Digit::new(split.next().unwrap()),
        ];
        Problem { digits, output }
    }
}

#[derive(PartialEq, Eq)]
struct Digit {
    // Wires are bytes corresponding to the ascii values 'a', 'b', 'c', etc.
    wires: Vec<u8>,
}

impl Digit {
    fn new(string: &str) -> Digit {
        Digit {
            wires: string.bytes().collect(),
        }
    }

    fn solve(&self, mapping: &[Vec<u8>; 10]) -> u8 {
        mapping
            .iter()
            .enumerate()
            .find(|(_, number)| {
                let mut set: HashSet<u8> = HashSet::from_iter(number.iter().cloned());
                for wire in &self.wires {
                    if !set.remove(wire) {
                        return false;
                    }
                }
                set.is_empty()
            })
            .unwrap()
            .0 as u8
    }
}

impl fmt::Debug for Digit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(str::from_utf8(&self.wires).unwrap())
    }
}

fn count_wire(problem: &Problem, target: u8) -> u32 {
    problem
        .digits
        .iter()
        .filter(|digit| digit.wires.iter().find(|wire| **wire == target).is_some())
        .count() as u32
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let problems: Vec<Problem> = input.trim().split('\n').map(Problem::new).collect();
    let count: u32 = problems
        .iter()
        .map(|problem| {
            problem
                .output
                .iter()
                .filter(|digit| {
                    digit.wires.len() == 2 // Digit 1.
                || digit.wires.len() == 4  // Digit 4.
                || digit.wires.len() == 3  // Digit 7.
                || digit.wires.len() == 7 // Digit 8.
                })
                .count() as u32
        })
        .sum();

    println!("The count of unique segments is {count}");

    let mut solutions: Vec<u32> = Vec::new();
    for problem in &problems {
        // First, find the digit #1:
        let one = problem
            .digits
            .iter()
            .find(|digit| digit.wires.len() == 2)
            .unwrap();

        // Then, find the digit #7:
        let seven = problem
            .digits
            .iter()
            .find(|digit| digit.wires.len() == 3)
            .unwrap();

        // Now, we know which wire corresponds to segment a:
        let a = *seven
            .wires
            .iter()
            .find(|wire| **wire != one.wires[0] && **wire != one.wires[1])
            .unwrap();

        // Segment c appears 8 times and segment f appears 9 times.
        // Find how many times the first wire in digit #1 appears.
        let count = count_wire(problem, one.wires[0]);
        let (c, f) = if count == 8 {
            (one.wires[0], one.wires[1])
        } else if count == 9 {
            (one.wires[1], one.wires[0])
        } else {
            panic!("Error! Unexpected count for segments c / f");
        };

        // Find the digit #4:
        let seven = problem
            .digits
            .iter()
            .find(|digit| digit.wires.len() == 4)
            .unwrap();

        // The 4th digit has segments b, d, c, and f. Find b and d.
        let b_and_d: Vec<u8> = seven
            .wires
            .iter()
            .copied()
            .filter(|wire| *wire != c && *wire != f)
            .collect();

        // Segment b appears 6 times, and segment d appears 7 times.
        let count = count_wire(problem, b_and_d[0]);
        let (b, d) = if count == 6 {
            (b_and_d[0], b_and_d[1])
        } else if count == 7 {
            (b_and_d[1], b_and_d[0])
        } else {
            panic!("Wrong count for segments b / d");
        };

        // We are missing e and g. e appears 4 times and g 7. e is unique, find that:
        let e = (b'a'..=b'g')
            .find(|wire| count_wire(problem, *wire) == 4)
            .unwrap();

        // The last segment is the one we haven't found yet.
        let g = (b'a'..=b'g')
            .find(|&wire| {
                wire != a && wire != b && wire != c && wire != d && wire != e && wire != f
            })
            .unwrap();

        println!("Segment a: {}", a as char);
        println!("Segment b: {}", b as char);
        println!("Segment c: {}", c as char);
        println!("Segment d: {}", d as char);
        println!("Segment e: {}", e as char);
        println!("Segment f: {}", f as char);
        println!("Segment g: {}", g as char);

        let mapping: [Vec<u8>; 10] = [
            vec![a, b, c, e, f, g],
            vec![c, f],
            vec![a, c, d, e, g],
            vec![a, c, d, f, g],
            vec![b, c, d, f],
            vec![a, b, d, f, g],
            vec![a, b, d, e, f, g],
            vec![a, c, f],
            vec![a, b, c, d, e, f, g],
            vec![a, b, c, d, f, g],
        ];

        solutions.push(
            problem
                .output
                .iter()
                .map(|digit| digit.solve(&mapping))
                .fold(0, |solution, digit| {
                    (solution * 10) + (digit as u32)
                }),
        )
    }

    let solution: u32 = solutions.iter().sum();
    println!("The solution is {}", solution);
}
