use std::fs;

fn parse_text() -> (Vec<(i64, i64)>, Vec<i64>) {
    let contents =
        fs::read_to_string("path")
            .expect("Should have been able to read the file");

    let ct: Vec<&str> = contents.trim().lines().collect();

    let mut ranges: Vec<(i64, i64)> = vec![];

    let mut idx = 0;

    for c in &ct {
        if *c == "" {
            break;
        }

        let range_val: Vec<i64> = c.split('-').map(|x| x.parse().unwrap_or(0)).collect();
        let range_tuple = (range_val[0], range_val[1]);
        ranges.push(range_tuple);

        idx += 1;
    }

    let ids = ct[idx + 1..]
        .iter()
        .map(|x| x.parse().unwrap_or(0))
        .collect::<Vec<i64>>();

    (ranges, ids)
}

// part 1
fn check_fresh(ranges: Vec<(i64, i64)>, ingredients: Vec<i64>) -> i64 {
    let mut total = 0;

    for ingredient in ingredients {
        let mut included = false;
        for range in &ranges {
            if range.0 <= ingredient && range.1 >= ingredient {
                included = true;
                break;
            }
        }

        if included {
            total += 1;
        }
    }

    total
}

// part 2
// merge ranges
fn reduce_ranges(ranges: Vec<(i64, i64)>, length: usize) -> Vec<(i64, i64)> {
    let mut new_ranges: Vec<(i64, i64)> = vec![];

    let mut i = 1;
    while i < length {
        let current = ranges.get(i).unwrap();
        let prev = ranges.get(i - 1).unwrap();

        if prev.1 >= current.0 {
            // zero-length ranges
            if current.0 == current.1 {
                new_ranges.push((prev.0, prev.1));
            } else {
                new_ranges.push((prev.0, cmp::max(current.1, prev.1)));
            }
            new_ranges.extend(&ranges[i + 1..]);

            break;
        } else {
            new_ranges.push(*prev);
            if i == length - 1 {
                new_ranges.push(*current);
            }
        }
        i += 1;
    }

    let new_length = new_ranges.len();

    if length == new_length || new_ranges.len() == 1 {
        new_ranges
    } else {
        reduce_ranges(new_ranges, new_length)
    }
}

// sort ranges by their first tuple element
fn construct_range(mut ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    ranges.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap());

    ranges
}

// get the sum of fresh ingredients
fn fresh_from_ranges(ranges: Vec<(i64, i64)>) -> i64 {
    let length = ranges.len();

    // sort (by x in (x,y)) and merge ranges
    let sorted_ranges = construct_range(ranges);
    let merged_ranges = reduce_ranges(sorted_ranges, length);

    let mut total = 0;

    for range in merged_ranges {
        total += range.1 - range.0 + 1;
    }

    total
}

fn main() {
    let parsed = parse_text();
    println!("{}", check_fresh(parsed.0, parsed.1)); // part 1
    println!("\n{}", fresh_from_ranges(parsed.0)); // pert 2
}
