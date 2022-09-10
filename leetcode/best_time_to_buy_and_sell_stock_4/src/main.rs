use std::cmp;

struct Solution {}

impl Solution {

    // profit[k, i] = max(profit[k, i - 1], prices[i] - prices[j] + dp[k - 1, j - 1])
    //
    // profit on day i with k transactions
    //  = max of
    //     1 if !transaction -> profit[k, i - 1]
    //     2 if  transaction -> profit[k - 1, i - 1]

    pub fn max_profit(k: usize, prices: Vec<i32>) -> i32 {
        let mut mins = vec![prices.first().copied().unwrap_or(0); k + 1];
        let mut profit = vec![0; k + 1];

        for i in 1..prices.len() {
            for k in 1..k + 1 {
                profit[k] = cmp::max(profit[k], prices[i] - mins[k]);
                mins[k] = cmp::min(mins[k], prices[i] - profit[k - 1]);
            }
        }

        profit[k]
    }
}

fn main() {
    let inputs = vec![
        (2, vec![2, 4, 1]),
        (2, vec![3, 2, 6, 5, 0, 3]),
        (1, vec![3, 2, 6, 5, 0, 3]),
    ];

    for (k, prices) in inputs {
        println!(
            "{} {:?} -> {}",
            k,
            prices.clone(),
            Solution::max_profit(k, prices.clone())
        );
    }
}
