use std::fs;

fn parse_input() -> Vec<Vec<String>> {
    fs::read_to_string("path")
        .expect("Should have been able to read the file")
        .split_whitespace()
        .map(|x| x.chars().map(|c| c.to_string()).collect())
        .collect::<Vec<Vec<String>>>()
}

fn make_beams(tachyon: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut split_tachyon = tachyon.clone();

    let rows = split_tachyon.len();
    let cols = split_tachyon[0].len();

    for r in 0..rows {
        for mut c in 0..cols {
            let current = tachyon[r].get(c).unwrap();

            match current.as_str() {
                "S" => split_tachyon[r + 1][c] = "|".to_owned(),
                "|" => {
                    if c < cols - 1 {
                        while tachyon[r][c + 1] == "." {
                            split_tachyon[r][c + 1] = "|".to_owned();
                            c += 1
                        }
                    }
                }
                "^" => {
                    if c != cols - 1 {
                        split_tachyon[r][c - 1] = "|".to_owned();
                        split_tachyon[r][c + 1] = "|".to_owned();
                    }
                }
                "." => {
                    if r > 1 && split_tachyon[r - 1][c] == "|".to_owned() {
                        split_tachyon[r][c] = "|".to_owned();
                    }
                }
                _ => continue,
            }
        }
    }

    split_tachyon
}

fn count_splits(split_tachyon: Vec<Vec<String>>) -> i32 {
    let rows = split_tachyon.len();
    let cols = split_tachyon[0].len();

    let mut total = 0;

    for r in 0..rows {
        for c in 0..cols {
            let current = split_tachyon[r].get(c).unwrap();

            match current.as_str() {
                "^" => {
                    if split_tachyon[r - 1][c] == "|" {
                        total += 1;
                    }
                }
                _ => continue,
            }
        }
    }

    total
}

fn count_timelines(beams: Vec<Vec<String>>) -> i64 {
    let mut beam = beams[0]
        .iter()
        .map(|x| if x == "S" { 1 } else { 0 })
        .collect::<Vec<i64>>();

    for row in beams {
        let mut new_beam = vec![0; beam.len()];
        for i in 0..beam.len() {
            if row[i] == "^" && beam[i] > 0 {
                new_beam[i - 1] += beam[i];
                new_beam[i + 1] += beam[i];
            } else {
                new_beam[i] += beam[i]
            }
        }
        beam = new_beam;
    }

    beam.iter().sum()
}

fn main() {
    let tachyon = parse_input();
    let beams = make_beams(tachyon);
    let count = count_splits(beams);

    let timelines = count_timelines(beams);
    println!("{timelines}");
}
