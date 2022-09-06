use std::collections::BTreeSet;
use std::cmp;

struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut max_sum, mut curr_sum) = (i32::MIN, 0);

        for num in nums {
            curr_sum += num;

            if curr_sum > max_sum {
                max_sum = curr_sum
            }

            if curr_sum < 0 {
                curr_sum = 0;
            }
        }

        max_sum
    }

    pub fn max_sub_array_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let (mut max_sum, mut curr_sum) = (i32::MIN, 0);

        let mut prefix_sums = BTreeSet::new();
        prefix_sums.insert(0);

        for num in nums {
            curr_sum += num;

            let smallest_iter = prefix_sums.range(curr_sum - k..).next();
            if let Some(prefix) = smallest_iter {
                max_sum = cmp::max(max_sum, curr_sum - prefix);

                if max_sum == k {
                    return max_sum
                }
            }

            prefix_sums.insert(curr_sum);
        }

        max_sum
    }
}

fn main() {
    let input = vec![
        (vec![1], 2),
        (vec![-2, 1, -3, 4, -1, 2, 1, -5, 4], 5),
        (vec![5, 4, -1, 7, 8], 20),
    ];

    for (vec, k) in input {
        println!("{:?} -> {}", vec.clone(), Solution::max_sub_array(vec.clone()));
        println!("{:?} {} -> {}", vec.clone(), k, Solution::max_sub_array_less_than_k(vec.clone(), k));
    }
}
