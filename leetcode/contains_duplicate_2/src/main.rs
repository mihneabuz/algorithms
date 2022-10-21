use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut seen = HashMap::with_capacity(nums.len());
        for (i, x) in nums.into_iter().enumerate() {
            if let Some(j) = seen.insert(x, i) {
                if i - j <= k as usize {
                    return true
                }
            }
        }

        false
    }
}

fn main() {
    let inputs = [
        (vec![1, 2, 3, 1], 3),
        (vec![1, 0, 1, 1], 1),
        (vec![1, 2, 3, 1, 2, 3], 2),
    ];

    for (nums, k) in inputs {
        println!("{}", Solution::contains_nearby_duplicate(nums, k));
    }
}
