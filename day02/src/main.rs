use std::fs;

struct Coordinates {
    depth: u32,
    position: u32,
    aim: u32,
}

impl Coordinates {
    fn new() -> Coordinates {
        Coordinates {
            depth: 0,
            position: 0,
            aim: 0,
        }
    }
    fn result(self) -> u32 {
        return self.depth * self.position;
    }
}

struct Command<'a> {
    command: &'a str,
    value: u32,
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").expect("Could not read input");
    let commands: Vec<Command> = file_contents
        .trim()
        .split('\n')
        .map(|line| line.split(' '))
        .map(|mut fragments| Command {
            command: fragments.next().expect("Improperly formatted input"),
            value: fragments
                .next()
                .expect("Improperly formatted input")
                .parse::<u32>()
                .expect("Expected a number"),
        })
        .collect();

    let first_solution = commands
        .iter()
        .fold(Coordinates::new(), |mut coordinates, command| {
            match command.command {
                "forward" => coordinates.position += command.value,
                "down" => coordinates.depth += command.value,
                "up" => coordinates.depth -= command.value,
                _ => panic!("Expected forward, up, or down"),
            };
            return coordinates;
        });
    println!("The first solution is: {}", first_solution.result());

    let second_solution = commands
        .iter()
        .fold(Coordinates::new(), |mut coordinates, command| {
            match command.command {
                "forward" => {
                    coordinates.position += command.value;
                    coordinates.depth += coordinates.aim * command.value;
                }
                "down" => coordinates.aim += command.value,
                "up" => coordinates.aim -= command.value,
                _ => panic!("Expected forward, up, or down"),
            };
            return coordinates;
        });
    println!("The second solution is: {}", second_solution.result());
}
