struct Solution {}

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let mut closest: i32 = nums.iter().take(3).sum();

        for (idx, num) in nums.iter().enumerate() {
            let (mut left, mut right) = (idx + 1, nums.len() - 1);

            while left < right {
                let sum = num + nums[left] + nums[right];

                if (target - sum).abs() < (target - closest).abs() {
                    closest = sum;
                }

                match target.cmp(&sum) {
                    std::cmp::Ordering::Equal => {
                        return target
                    },

                    std::cmp::Ordering::Less => {
                        right -= 1;
                    },

                    std::cmp::Ordering::Greater => {
                        left += 1;
                    },
                }
            }
        }

        closest
    }
}

fn main() {
    let inputs = [(vec![-1, 2, 1, -4], 1), (vec![0, 0, 0], 1)];

    for (nums, target) in inputs {
        println!("{}", Solution::three_sum_closest(nums, target));
    }
}
