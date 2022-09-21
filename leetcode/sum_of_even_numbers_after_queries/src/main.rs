struct Solution {}

impl Solution {
    pub fn sum_even_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = Vec::with_capacity(nums.len());
        let mut sum_even = nums.iter().filter(|x| **x % 2 == 0).sum();

        for (val, idx) in queries.into_iter().map(|q| (q[0], q[1])) {
            let mut num = nums[idx as usize];

            if num % 2 == 0 {
                sum_even -= num;
            }

            num += val;

            if num % 2 == 0 {
                sum_even += num;
            }

            nums[idx as usize] = num;

            res.push(sum_even);
        }

        res
    }
}

fn main() {
    let inputs = [
        (vec![1, 2, 3, 4], vec![vec![1, 0], vec![-3, 1], vec![-4, 0], vec![2, 3]]),
        (vec![1], vec![vec![4, 0]])
    ];

    for (nums, queries) in inputs {
        println!("{:?}", Solution::sum_even_after_queries(nums, queries));
    }
}
