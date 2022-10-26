use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut map = HashMap::from([(0, 0)]);

        let mut sum = 0;
        for (i, num) in nums.into_iter().enumerate() {
            sum += num;

            if let Some(j) = map.get(&(sum % k)) {
                if i - j > 0 {
                    return true;
                }
            }

            map.entry(sum % k).or_insert(i + 1);
        }

        false
    }

    pub fn check_subarray_sum_prefix(nums: Vec<i32>, k: i32) -> bool {
        let n = nums.len();

        let mut prefix = vec![0; n];
        let mut sum = 0;
        for (i, x) in nums.iter().enumerate() {
            sum += x;
            prefix[i] = sum;

            if sum % k == 0 && i > 0 {
                return true;
            }
        }

        for i in 1..n {
            for j in i + 1..n {
                if (prefix[j] - prefix[i - 1]) % k == 0 {
                    return true;
                }
            }
        }

        false
    }
}

fn main() {
    let inputs = [
        (vec![23, 2, 4, 6, 7], 6),
        (vec![23, 2, 6, 4, 7], 6),
        (vec![23, 2, 6, 4, 7], 13),
        (vec![0], 1),
        (vec![0, 0], 1),
        (vec![1, 0], 2),
    ];

    for (nums, k) in inputs {
        println!("{}", Solution::check_subarray_sum(nums, k));
    }
}
