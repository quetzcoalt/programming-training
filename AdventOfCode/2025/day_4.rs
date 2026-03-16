use std::fs;

fn parse_text() -> Vec<Vec<u32>> {
    let ct = fs::read_to_string(path)
        .expect("Should have been able to read the file")
        .trim()
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let mut content: Vec<Vec<u32>> = vec![];

    for c in ct {
        let element = c
            .trim()
            .chars()
            .map(|x| if x == '@' { 1 } else { 0 })
            .collect();

        content.push(element);
    }

    content
}

// ---------- part 1
fn find_accessible(rolls: Vec<Vec<u32>>) -> i32 {
    let rows = rolls.len();
    let cols = rolls[0].len();

    let mut rolls_count = 0;

    let directions: [(i32, i32); 8] = [
        (0, 1),   // bottom
        (0, -1),  // up
        (1, 0),   // right
        (-1, 0),  // left
        (1, 1),   // bottom-right
        (-1, -1), // up-left
        (1, -1),  // up-right
        (-1, 1),  // left-up
    ];

    for r_idx in 0..rows {
        for c_idx in 0..cols {
            if rolls[r_idx][c_idx] == 1 {
                let mut all = 0;

                for d in directions {
                    let row_idx = r_idx as i32 + d.1 as i32;
                    let col_idx = c_idx as i32 + d.0;

                    if row_idx >= 0
                        && row_idx < rows as i32
                        && col_idx >= 0
                        && col_idx < cols as i32
                    {
                        if rolls[row_idx as usize][col_idx as usize] == 1 {
                            all += 1;
                        }
                    }
                }

                if all < 4 {
                    rolls_count += 1;
                }
            }
        }
    }

    rolls_count
}

// ---------- part 2

// find the number of removable rolls 
fn total_removable_rolls(rolls: Vec<Vec<u32>>, rolls_removed: i32) -> i32 {
    let accessibles = find_accessible(rolls.clone());

    if accessibles.len() == 0 {
        return rolls_removed;
    } else {
        let new_rolls = remove_rolls(rolls);
        return total_removable_rolls(new_rolls, rolls_removed + accessibles.len() as i32);
    }
}

// subtract accessible rolls from the grid
fn remove_rolls(rolls: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let accessibles = find_accessible(rolls.clone());
    let mut new_rolls = rolls.clone();

    for accessible in accessibles {
        new_rolls[accessible.0 as usize][accessible.1 as usize] = 0
    }

    new_rolls
}

// get list of accessible indices -> Vec of tuple of (x,y)
fn find_accessibles(rolls: Vec<Vec<u32>>) -> Vec<(i32, i32)> {
    let rows = rolls.len();
    let cols = rolls[0].len();

    let directions: [(i32, i32); 8] = [
        (0, 1),   // bottom
        (0, -1),  // up
        (1, 0),   // right
        (-1, 0),  // left
        (1, 1),   // bottom-right
        (-1, -1), // up-left
        (1, -1),  // up-right
        (-1, 1),  // left-up
    ];

    let mut to_be_removed: Vec<(i32, i32)> = vec![];

    for r_idx in 0..rows {
        for c_idx in 0..cols {
            if rolls[r_idx][c_idx] == 1 {
                let mut all = 0;

                for d in directions {
                    let row_idx = r_idx as i32 + d.1 as i32;
                    let col_idx = c_idx as i32 + d.0;

                    if row_idx >= 0
                        && row_idx < rows as i32
                        && col_idx >= 0
                        && col_idx < cols as i32
                    {
                        if rolls[row_idx as usize][col_idx as usize] == 1 {
                            all += 1;
                        }
                    }
                }

                if all < 4 {
                    to_be_removed.push((r_idx as i32, c_idx as i32));
                }
            }
        }
    }

    to_be_removed
}

fn main() {
    let rolls = parse_text();
    let accessible_rolls = find_accessible(rolls);
    println!("{}", accessible_rolls);
}
