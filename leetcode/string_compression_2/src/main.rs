struct Solution {}

impl Solution {
    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let k = k as usize;

        let mut dp = vec![vec![n as i32; k + 1]; n + 1];
        dp[0][0] = 0;

        for i in 1..=n {
            for k in 0..=k {
                if k > 0 {
                    dp[i][k] = dp[i - 1][k - 1]
                }

                let (mut removed, mut count) = (0, 0);
                for ptr in (0..i).rev() {
                    if bytes[ptr] == bytes[i - 1] {
                        count += 1;
                    } else {
                        removed += 1;
                        if removed > k {
                            break;
                        }
                    }

                    dp[i][k] = std::cmp::min(dp[i][k], dp[ptr][k - removed] + Self::block(count));
                }
            }
        }

        dp[n][k]
    }

    fn block(count: u8) -> i32 {
        match count {
            0 => 0,
            1 => 1,
            2..=9 => 2,
            10..=99 => 3,
            _ => 4
        }
    }
}

fn main() {
    let inputs = [("aaabcccd", 2), ("aabbaa", 2), ("aaaaaaaaaaa", 0)];

    for (s, k) in inputs {
        println!("{}", Solution::get_length_of_optimal_compression(String::from(s), k));
    }
}
