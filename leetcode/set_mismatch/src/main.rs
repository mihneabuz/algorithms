struct Solution {}

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let (mut twice, mut missing) = (-1, -1);

        let mut seen = vec![false; nums.len()];
        for num in nums {
            if seen[num as usize - 1] {
                twice = num;
            } else {
                seen[num as usize - 1] = true;
            }
        }

        for (num, s) in seen.into_iter().enumerate() {
            if !s {
                missing = num as i32 + 1;
                break;
            }
        }

        vec![twice, missing]
    }
}

fn main() {
    let inputs = [vec![1, 2, 2, 4], vec![1, 1]];

    for input in inputs {
        println!("{:?}", Solution::find_error_nums(input));
    }
}
