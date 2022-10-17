use std::cmp;

struct Solution {}

impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        Self::min_difficulty_iterative(&job_difficulty, d as usize)
    }

    pub fn min_difficulty_iterative(diff: &[i32], maxd: usize) -> i32 {
        let n = diff.len();

        let mut dp = vec![vec![-1; maxd + 1]; diff.len() + 1];
        dp[diff.len()][0] = 0;

        for i in (0..n).rev() {
            for d in (0..cmp::min(maxd, n - i)).map(|d| d + 1) {
                let (mut hardest, mut best, mut valid) = (0, i32::MAX, false);
                for j in i..(n - d + 1) {
                    hardest = cmp::max(hardest, diff[j]);

                    let cost = dp[j + 1][d - 1];
                    if cost != -1 {
                        valid = true;
                        best = cmp::min(best, hardest + cost);
                    }
                }

                if valid {
                    dp[i][d] = best;
                }
            }
        }

        dp[0][maxd]
    }

    pub fn min_difficulty_memoized(i: usize, d: usize, diff: &[i32], memo: &mut [Vec<i32>]) -> i32 {
        if d == 0 && i == diff.len() {
            return 0
        }

        if d == 0 || i == diff.len() || diff.len() - i < d {
            return -1
        }

        if memo[d][i] != -2 {
            return memo[d][i]
        }

        let (mut hardest, mut best, mut valid) = (0, i32::MAX, false);
        for j in i..diff.len() {
            hardest = cmp::max(hardest, diff[j]);

            let cost = Self::min_difficulty_memoized(j + 1, d - 1, &diff, memo);
            if cost != -1 {
                valid = true;
                best = cmp::min(best, hardest + cost);
            }
        }

        let res = if valid { best } else { -1 };
        memo[d][i] = res;
        res
    }

    pub fn min_difficulty_backtrack(i: usize, d: usize, diff: &[i32]) -> i32 {
        // we have no more jobs AND no more days -> we are done
        if d == 0 && i == diff.len() {
            return 0
        }

        // we have no more days, no more jobs, or more days than jobs
        if d == 0 || i == diff.len() || diff.len() - i < d {
            return -1
        }

        let (mut hardest, mut best, mut valid) = (0, i32::MAX, false);
        for j in i..diff.len() {
            hardest = cmp::max(hardest, diff[j]);

            let cost = Self::min_difficulty_backtrack(j + 1, d - 1, &diff);
            if cost != -1 {
                valid = true;
                best = cmp::min(best, hardest + cost);
            }
        }

        if valid { best } else { -1 }
    }

}

fn main() {
    let inputs = [
        (vec![6, 5, 4, 3, 2, 1], 2),
        (vec![9, 9, 9], 4),
        (vec![1, 1, 1], 3),
    ];

    for (diff, d) in inputs {
        println!("{}", Solution::min_difficulty(diff, d));
    }
}
