use std::fs;

// Returns a vector holding a 1 in the nth position if 1
// is the nth most common bit, 0 otherwise.
fn calculate_bit_count(slice: &Vec<&str>) -> Vec<bool> {
    let byte_size = slice[0].trim().len();

    return slice
        .iter()
        .fold(vec![0; byte_size], |mut aggregated, &line| {
            for index in 0..byte_size {
                if line.as_bytes()[index] == b'0' {
                    aggregated[index] -= 1;
                } else {
                    aggregated[index] += 1;
                }
            }
            return aggregated;
        })
        .iter()
        .map(|bit_count| {
            return *bit_count >= 0;
        })
        .collect();
}

fn byte_string_to_integer(string: &Vec<bool>) -> u64 {
    let mut integer: u64;
    if string[0] {
        integer = 1;
    } else {
        integer = 0;
    }
    for index in 1..string.len() {
        integer <<= 1;
        if string[index] {
            integer |= 1;
        }
    }
    return integer;
}

fn main() {
    // Common.
    let file = fs::read_to_string("input.txt").expect("Could not open file");
    let input: Vec<&str> = file.trim().split('\n').collect();

    // Part 1.
    let bit_count_vector = calculate_bit_count(&input);
    let mut mask: u64 = 1;
    for _ in 1..bit_count_vector.len() {
        mask = (mask << 1) | 1;
    }
    let gamma_rate = byte_string_to_integer(&bit_count_vector);
    let epsilon_rate = (!gamma_rate) & mask;
    println!("Part 1:");
    println!("  Gamma rate: {gamma_rate}");
    println!("Epsilon rate: {epsilon_rate}");
    println!("    Solution: {}", gamma_rate * epsilon_rate);

    // Part 2.
    let mut oxygen_count = input.clone();
    {
        let mut index = 0;
        while oxygen_count.len() > 1 {
            let bit_count_vector = calculate_bit_count(&oxygen_count);
            oxygen_count = oxygen_count
                .into_iter()
                .filter(|&line| {
                    let line = line.as_bytes();
                    return bit_count_vector[index] && line[index] == b'1'
                        || !bit_count_vector[index] && line[index] == b'0';
                })
                .collect();
            index += 1;
        }
    }

    let mut co2_scrubber_count = input.clone();
    {
        let mut index = 0;
        while co2_scrubber_count.len() > 1 {
            let bit_count_vector = calculate_bit_count(&co2_scrubber_count);
            co2_scrubber_count = co2_scrubber_count
                .into_iter()
                .filter(|&line| {
                    let line = line.as_bytes();
                    return bit_count_vector[index] && line[index] == b'0'
                        || !bit_count_vector[index] && line[index] == b'1';
                })
                .collect();
            index += 1;
        }
    }

    let oxygen_count = u64::from_str_radix(oxygen_count[0], 2).expect("Not binary");
    let co2_scrubber_count = u64::from_str_radix(co2_scrubber_count[0], 2).expect("Not binary");
    println!("\nPart 2:");
    println!("      Oxygen count: {oxygen_count}");
    println!("CO2 scrubber count: {co2_scrubber_count}");
    println!("            Result: {}", oxygen_count * co2_scrubber_count);
}
