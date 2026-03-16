use std::{cmp, collections::HashMap};

fn lilys_homework(arr: &[i32]) -> i32 {
    let length = arr.len();

    let mut original_positions = HashMap::new();

    for (index, &value) in arr.iter().enumerate() {
        original_positions.insert(value, index);
    }

    let mut sorted_asc = arr.to_vec();
    sorted_asc.sort();

    let mut sorted_desc = sorted_asc.clone();
    sorted_desc.reverse();

    let asc_swaps = count_swaps(&sorted_asc, &original_positions, length);
    let desc_swaps = count_swaps(&sorted_desc, &original_positions, length);

    cmp::min(asc_swaps, desc_swaps)
}

fn count_swaps(target: &[i32], original_positions: &HashMap<i32, usize>, length: usize) -> i32 {
    let mut visited = vec![false; length];

    let mut total_swaps = 0;

    for i in 0..length {
        if !visited[i] {
            let mut cycle_size = 0;
            let mut current = i;

            while !visited[current] {
                visited[current] = true;
                cycle_size += 1;

                // Find where the element at current position should go
                let current_value = target[current];
                let original_pos = original_positions[&current_value];
                current = original_pos;
            }

            if cycle_size > 1 {
                total_swaps += (cycle_size - 1) as i32;
            }
        }
    }

    total_swaps
}
