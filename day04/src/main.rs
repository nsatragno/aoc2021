use std::fs;

#[derive(Copy, Clone)]
struct Spot {
    number: u32,
    filled: bool,
}

impl Spot {
    fn new(number: u32) -> Spot {
        Spot {
            number,
            filled: false,
        }
    }
}

fn check_winner(board: &mut [[Spot; 5]; 5]) -> bool {
    // Check for a horizontal bingo.
    'next: for x in 0..board.len() {
        for y in 0..board.len() {
            if !board[x][y].filled {
                continue 'next;
            }
        }
        // Found a horizontal winner.
        return true;
    }

    // Check for a vertical bingo.
    'next: for y in 0..board.len() {
        for x in 0..board.len() {
            if !board[x][y].filled {
                continue 'next;
            }
        }
        // Found a horizontal winner.
        return true;
    }
    return false;
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Could not open file");
    let mut lines = file.trim().split('\n');
    let numbers: Vec<u32> = lines
        .next()
        .unwrap()
        .trim()
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect();

    let mut boards: Vec<[[Spot; 5]; 5]> = Vec::new();

    // Consume the first newline and check if we're done.
    while lines.next().is_some() {
        let mut board = [[Spot::new(0); 5]; 5];
        for x in 0..5 {
            lines
                .next()
                .unwrap()
                .trim()
                .split(' ')
                .filter(|&maybe_number| !maybe_number.trim().is_empty()) // Remove whitespace.
                .map(|number| Spot::new(number.trim().parse().unwrap()))
                .enumerate()
                .for_each(|(y, number)| board[x][y] = number);
        }
        boards.push(board);
    }

    for number in numbers.as_slice() {
        let mut boards_to_remove : Vec<usize> = Vec::new();
        for (i, board) in boards.iter_mut().enumerate() {
            let mut has_slot = false;
            'done: for line in board.iter_mut() {
                for slot in line.iter_mut() {
                    if slot.number == *number {
                        slot.filled = true;
                        has_slot = true;
                        break 'done;
                    }
                }
            }
            if has_slot {
                if check_winner(board) {
                    let sum = board.iter().fold(0, |sum, line| {
                        sum + line
                            .iter()
                            .filter(|slot| !slot.filled)
                            .fold(0, |sum, slot| sum + slot.number)
                    });
                    for line in board {
                        for spot in line {
                            print!("{:?} ", spot.number);
                        }
                        println!("");
                    }
                    boards_to_remove.push(i);
                    println!("Found a winner:");
                    println!("  The last number is {number}");
                    println!("  The winning sum is {sum}");
                    println!("The winning score is {}", number * sum);
                    println!("");
                    println!("");
                }
            }
        }

        boards_to_remove.reverse();
        for i in boards_to_remove {
            boards.swap_remove(i);
        }
    }
}
