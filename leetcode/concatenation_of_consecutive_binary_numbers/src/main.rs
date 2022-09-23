struct Solution {}

const MOD: u64 = 10u64.pow(9) + 7;

impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        (1..=n).fold(0u64, |acc, i| ((acc << 32 - i.leading_zeros()) + i as u64) % MOD) as i32
    }
}

fn main() {
    let inputs = [1, 3, 12];

    for input in inputs {
        println!("{}", Solution::concatenated_binary(input));
    }
}
