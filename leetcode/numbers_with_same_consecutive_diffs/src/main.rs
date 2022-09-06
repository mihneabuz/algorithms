struct Solution {}

trait Digit {
    fn is_digit(&self) -> bool;
}

impl Digit for i32 {
    fn is_digit(&self) -> bool {
        *self >= 0 && *self < 10
    }
}

impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        if k == 0 {
            (1..10).into_iter().map(|x| Self::repeat(x, n)).collect()
        } else {
            let mut results = Vec::new();

            for i in 1..10 {
                Self::build_number(i, 1, n, k, &mut results);
            }

            results
        }
    }

    fn build_number(x: i32, digits: i32, n: i32, k: i32, results: &mut Vec<i32>) {
        if digits == n {
            results.push(x);
        } else {
            for next in [x % 10 + k, x % 10 - k].iter().filter(|x| x.is_digit()) {
                Self::build_number(x * 10 + next, digits + 1, n, k, results);
            }
        }
    }

    fn repeat(digit: i32, n: i32) -> i32 {
        (0..n).into_iter().fold(0, |acc, _| acc * 10 + digit)
    }
}

fn main() {
    println!("{} {} -> {:?}", 3, 7, Solution::nums_same_consec_diff(3, 7));
    println!("{} {} -> {:?}", 2, 1, Solution::nums_same_consec_diff(2, 1));
    println!("{} {} -> {:?}", 2, 0, Solution::nums_same_consec_diff(2, 0));
    println!("{} {} -> {:?}", 3, 1, Solution::nums_same_consec_diff(3, 1));
    println!("{} {} -> {:?}", 5, 0, Solution::nums_same_consec_diff(3, 1));
}
