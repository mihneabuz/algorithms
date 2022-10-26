use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;

        let mut map = HashMap::new();
        map.insert(0, 1);

        let mut sum = 0;
        for num in nums {
            sum += num;

            if let Some(c) = map.get(&(sum - k)) {
                count += c;
            }

            map.entry(sum).and_modify(|c| *c += 1).or_insert(1);
        }

        count
    }

    pub fn subarray_sum_prefix(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut count = 0;

        let mut prefix = vec![0; n];
        nums.iter().enumerate().fold(0, |mut acc, (i, x)| {
            acc += x;
            prefix[i] = acc;

            if acc == k {
                count += 1;
            }

            acc
        });

        for i in 1..n {
            for j in i..n {
                if prefix[j] - prefix[i - 1] == k {
                    count += 1;
                }
            }
        }

        count
    }
}

fn main() {
    let inputs = [(vec![1, 1, 1], 2), (vec![1, 2, 3], 3), (vec![-1, -1, 1], 0)];

    for (nums, k) in inputs {
        println!("{}", Solution::subarray_sum(nums, k));
    }
}
