use std::collections::HashMap;

fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut frequency = HashMap::new();

    nums.into_iter()
        .for_each(|num| *frequency.entry(num).or_insert(0) += 1);

    let mut frequent: Vec<(i32, i32)> = frequency.into_iter().collect();
    frequent.sort_by(|a, b| b.1.cmp(&a.1));
    frequent[0..k as usize]
        .iter()
        .take(k as usize)
        .map(|x| x.0)
        .collect()
}
