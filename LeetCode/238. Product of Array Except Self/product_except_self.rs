// bruteforce
fn product_except_self_brute_force(nums: Vec<i32>) -> Vec<i32> {
    let mut answer: Vec<i32> = vec![];

    for i in 0..nums.len() {
        let mut product = 1;
        for (j, k) in nums.iter().enumerate() {
            if i != j {
                product *= k;
            }
        }
        answer.push(product);
    }

    answer
}


// Optimized
fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let length = nums.len();

    let mut prefix: Vec<i32> = vec![1; length];
    let mut suffix: Vec<i32> = vec![1; length];
    let mut answer: Vec<i32> = vec![1; length];

    // make prefix vector
    prefix[0] = nums[0];
    for i in 1..length {
        prefix[i] = nums[i] * prefix[i - 1];
    }

    // make suffix vector
    suffix[length - 1] = nums[length - 1];
    for i in (0..(length - 1)).rev() {
        suffix[i] = nums[i] * suffix[i + 1];
    }

    for i in 0..length {
        if i == 0 {
            answer[i] = suffix[i + 1]
        } else if i == length - 1 {
            answer[i] = prefix[i - 1]
        } else {
            answer[i] = prefix[i - 1] * suffix[i + 1];
        }
    }

    answer
}
