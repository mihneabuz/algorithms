use std::cmp;

struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() == 0 { return 0; }

        let mut mins = (prices[0], prices[0]);
        let mut profit = (0, 0);

        for i in 1..prices.len() {
            profit.0 = cmp::max(profit.0, prices[i] - mins.0);
            mins.0 = cmp::min(mins.0, prices[i]);

            profit.1 = cmp::max(profit.1, prices[i] - mins.1);
            mins.1 = cmp::min(mins.1, prices[i] - profit.0);
        }

        profit.1
    }
}

fn main() {
    let inputs = vec![
        (vec![3, 2, 6, 5, 0, 3], 7),
        (vec![7, 6, 5, 4, 3, 1], 0),
        (vec![3, 3, 5, 0, 0, 3, 1, 4], 6)
    ];

    for (prices, res) in inputs {
        println!("{:?} -> {} | {}", &prices, Solution::max_profit(prices.clone()), res);
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn empty() {
        assert_eq!(Solution::max_profit(vec![]), 0);
    }

    #[test]
    fn simple() {
        assert_eq!(Solution::max_profit(vec![2, 4, 1]), 2);
    }

    #[test]
    fn complex() {
        let inputs = vec![
            (vec![3, 2, 6, 5, 0, 3], 7),
            (vec![7, 6, 5, 4, 3, 1], 0),
            (vec![3, 3, 5, 0, 0, 3, 1, 4], 6)
        ];

        for (prices, res) in inputs {
            assert_eq!(Solution::max_profit(prices), res);
        }
    }
}
