use std::fs;

enum ParseResult {
    Ok,
    UnexpectedClosing { char: char },
    TooManyChars { stack: Vec<char> },
}

fn parse(line: &str) -> ParseResult {
    let mut stack: Vec<char> = Vec::new();
    for char in line.chars() {
        match char {
            '(' | '[' | '{' | '<' => stack.push(char),
            ')' => {
                if stack.pop() != Some('(') {
                    return ParseResult::UnexpectedClosing { char };
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return ParseResult::UnexpectedClosing { char };
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return ParseResult::UnexpectedClosing { char };
                }
            }
            '>' => {
                if stack.pop() != Some('<') {
                    return ParseResult::UnexpectedClosing { char };
                }
            }
            _ => panic!("Unknown character {}", char),
        }
    }
    if stack.is_empty() {
        ParseResult::Ok
    } else {
        stack.reverse();
        ParseResult::TooManyChars { stack }
    }
}

fn score(line: &str) -> u32 {
    match parse(line) {
        ParseResult::UnexpectedClosing { char } => match char {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("Unknown character {}", char),
        },
        _ => 0,
    }
}

fn autocomplete(line: &str) -> Vec<char> {
    match parse(line) {
        ParseResult::TooManyChars { stack } => stack,
        _ => Vec::new(),
    }
}

fn score_autocomplete(chars: Vec<char>) -> u64 {
    chars.iter().fold(0, |score, char| {
        score * 5
            + match char {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!("Unexpected character {char}"),
            }
    })
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let solution: u32 = file.trim().split('\n').map(str::trim).map(score).sum();
    println!("First part:");
    println!("The solution is {}", solution);

    let mut solutions: Vec<u64> = file
        .trim()
        .split('\n')
        .map(str::trim)
        .map(autocomplete)
        .map(score_autocomplete)
        .filter(|a| *a != 0)
        .collect();
    solutions.sort_unstable();
    println!("Second part:");
    println!("The solution is {}", solutions[solutions.len() / 2]);
}
