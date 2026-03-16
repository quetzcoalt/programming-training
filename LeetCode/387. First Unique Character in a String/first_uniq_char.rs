use std::collections::HashMap;

fn first_uniq_char(s: String) -> i32 {
    let mut char_indices: HashMap<char, i32> = HashMap::new();

    for c in s.chars() {
        *char_indices.entry(c).or_insert(0) += 1;
    }

    for (i, c) in s.chars().enumerate() {
        if char_indices[&c] == 1 {
            return i as i32;
        }
    }
    -1
}
