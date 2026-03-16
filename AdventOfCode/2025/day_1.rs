use std::fs;

fn parse_text() -> Vec<String> {
    let contents = fs::read_to_string("path")
        .expect("Should have been able to read the file");

    contents
        .trim()
        .split("\r\n")
        .map(|v| v.to_string())
        .collect()
}

// part 1
fn get_zeroes(rotations: Vec<String>) -> i32 {
    let mut start = 50;
    let mut count = 0;
    let dial = 100;

    for rotation in rotations {
        let direction = rotation.chars().next().unwrap_or(' ');
        let amount: i32 = rotation[1..].parse().unwrap_or(0);

        match direction {
            'L' => {
                if amount >= start && start != 0 {
                    count += (amount - start - 1) / dial + 1
                }
                start = (start - amount).rem_euclid(dial)
            }
            'R' => {
                count += (start + amount) / dial;
                start = (start + amount).rem_euclid(dial)
            }
            _ => panic!("Invalid rotation direction"),
        }
    }

    count
}

// part 2
fn get_zeroes_2(rotations: Vec<String>) -> i32 {
    let mut start = 50;
    let mut count = 0;
    let dial = 100;

    for rotation in rotations {
        let direction = rotation.chars().next().unwrap_or(' ');
        let amount: i32 = rotation[1..].parse().unwrap_or(0);

        for i in 0..amount {
            if direction == 'L' {
                start -= 1
            } else {
                start += 1
            }
            if start % 100 == 0 {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let rotations = parse_text();
    let zeroes = get_zeroes(rotations);
    println!("{}", zeroes);
}
