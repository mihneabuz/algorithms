use std::cmp;
use std::collections::HashMap;

struct Solution {}

type Cache = HashMap<(usize, usize), i32>;

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let n = multipliers.len();

        let mut dp: Vec<Vec<i32>> = (0..=n).map(|i| vec![0; n + 1 - i]).collect();

        for i in 0..n {
            dp[i + 1][0] = dp[i][0] + multipliers[i] * nums[i];
            dp[0][i + 1] = dp[0][i] + multipliers[i] * nums[nums.len() - 1 - i];
        }

        for i in 1..=n {
            for j in 1..=n - i {
                let mult = multipliers[i + j - 1];
                dp[i][j] = cmp::max(
                    dp[i - 1][j] + mult * nums[i - 1],
                    dp[i][j - 1] + mult * nums[nums.len() - j]
                );
            }
        }

        (0..=n).map(|i| dp[i][n - i]).max().unwrap_or(0)
    }

    pub fn maximum_score_top_down_cached(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let n = multipliers.len();

        let mut cache = (0..=n).map(|i| [
            ((i, 0), nums[..i].iter().zip(multipliers.iter()).map(|(x, m)| x * m).sum()),
            ((0, i), nums[nums.len() - i..].iter().rev().zip(multipliers.iter()).map(|(x, m)| x * m).sum())
        ]).flatten().collect();

        (0..=n).map(|i| Solution::top_down_cached(&nums, &multipliers, i, n - i, &mut cache)).max().unwrap_or(0)
    }

    fn top_down_cached(nums: &[i32], mults: &[i32], i: usize, j: usize, cache: &mut Cache) -> i32 {
        if let Some(val) = cache.get(&(i, j)) {
            return *val
        }

        let mult = mults[i + j - 1];
        let res = cmp::max(
            nums[i - 1] * mult + Solution::top_down_cached(&nums, &mults, i - 1, j, cache),
            nums[nums.len() - j] * mult + Solution::top_down_cached(&nums, &mults, i, j - 1, cache)
        );

        cache.insert((i, j), res);

        res
    }

    pub fn maximum_score_top_down(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let n = multipliers.len();
        (0..=n).map(|i| Solution::top_down(&nums, &multipliers, i, n - i)).max().unwrap_or(0)
    }

    fn top_down(nums: &[i32], mults: &[i32], i: usize, j: usize) -> i32 {
        if i == 0 {
            return nums[nums.len() - j..].iter().rev().zip(mults.iter()).map(|(x, m)| x * m).sum() 
        }

        if j == 0 {
            return nums[..i].iter().zip(mults.iter()).map(|(x, m)| x * m).sum()
        }

        let pick_left = nums[i - 1] * mults[i + j - 1];
        let pick_right = nums[nums.len() - j] * mults[i + j - 1];
        cmp::max(
            pick_left + Solution::top_down(&nums, &mults, i - 1, j),
            pick_right + Solution::top_down(&nums, &mults, i, j - 1) 
        )
    }

    pub fn maximum_score_bottom_up(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let mut solutions = Vec::new();
        Solution::bottom_up(&nums, &multipliers, 0, &mut solutions);
        solutions.into_iter().max().unwrap_or(0)
    }

    fn bottom_up(nums: &[i32], mults: &[i32], acc: i32, solutions: &mut Vec<i32>) {
        if mults.len() == 0 {
            solutions.push(acc);
            return;
        }

        let n = nums.len() - 1;
        Solution::bottom_up(&nums[1..], &mults[1..], acc + nums[0] * mults[0], solutions);
        Solution::bottom_up(&nums[..n], &mults[1..], acc + nums[n] * mults[0], solutions);
    }
}

fn main() {
    let inputs = [
        (vec![1, 2, 3], vec![3, 2, 1]),
        (vec![1, 2, 3, 4, 5], vec![3, 2, 1]),
        (vec![-5, -3, -3, -2, 7, 1], vec![-10, -5, 3, 4, 6]),
    ];

    for (nums, mults) in inputs {
        println!("{}", Solution::maximum_score(nums, mults));
    }
}
