use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let length = nums.len();

        if length == 2 {
            return vec![0, 1];
        }

        let mut map = HashMap::new();

        for i in 0..length {
            let num = target - nums[i];

            if map.contains_key(&num) {
                return vec![*map.get_key_value(&num).unwrap().1, i as i32];
            } else {
                map.insert(nums[i], i as i32);
            }
        }

        vec![]
    }
}
