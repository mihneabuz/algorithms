use std::collections::HashMap;
use std::cmp;

struct Solution {}

const MOD: i32 = 10i32.pow(9) + 7;

impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        Solution::num_rolls_to_target_dp(n as usize, k as usize, target as usize)
    }

    pub fn num_rolls_to_target_dp(n: usize, k: usize, target: usize) -> i32 {
        let mut dp = vec![vec![0i32; target + 1]; n];

        for i in 1..=cmp::min(k, target) {
            dp[0][i] = 1;
        }

        for i in 1..n {
            for t in 1..=target {
                if k * (i + 1) < t {
                    break;
                }

                for k in 1..=cmp::min(k, t - 1) {
                    dp[i][t] += dp[i - 1][t - k];
                    dp[i][t] %= MOD;
                }
            }
        }

        dp[n - 1][target] % MOD
    }

    pub fn num_rolls_to_target_recursive(n: i32, k: i32, target: i32) -> i32 {
        if target == 0 {
            return 1;
        }

        if n == 0 {
            return 0;
        }

        (1..=k).fold(0, |acc, dice| {
            if dice <= target {
                acc + Solution::num_rolls_to_target(n - 1, k, target - dice)
            } else {
                acc
            }
        })
    }

    pub fn num_rolls_to_target_cached(n: i32, k: i32, target: i32, cache: &mut HashMap<(i32, i32), i32>) -> i32 {
        if let Some(res) = cache.get(&(target, n)) {
            return *res;
        }

        let res = match (target, n) {
            (0, _) => 1,

            (_, 0) => 0,

            _ => (1..=k).fold(0, |acc, dice| {
                    if dice <= target {
                        acc + Solution::num_rolls_to_target(n - 1, k, target - dice)
                    } else {
                        acc
                    }
                })
        };

        cache.insert((target, n), res);

        res
    }
}

/*
* 2 6 7
*
* 0 0 0 0 0 0 0
* 0 0 0 0 0 0 0
*
* 0 1 1 1 1 1 1 0
* 0 1 1 2 4 4 4 2
*
*/

fn main() {
    let inputs = [
        (1, 6, 3),
        (2, 6, 7),
        (30, 30, 500)
    ];

    for (n, k, target) in inputs {
        println!("{:?}", Solution::num_rolls_to_target(n, k, target));
    }
}
