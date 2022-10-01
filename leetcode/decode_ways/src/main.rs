struct Solution {}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        Solution::decode_dp(&s)
    }

    fn decode_dp(s: &str) -> i32 {
        let mut dp = vec![0; s.len() + 1];
        let mut iter = s.bytes().map(|c| c - '0' as u8).rev().enumerate();

        let (_, mut last_c) = iter.next().unwrap();
        dp[0] = 1;
        dp[1] = if last_c != 0 { 1 } else { 0 };

        for (i, c) in iter {
            if c == 0 {
                continue;
            }

            let x = c * 10 + last_c;

            if x > 0 && x <= 26 {
                dp[i + 1] = dp[i] + dp[i - 1];
            } else {
                dp[i + 1] = dp[i];
            }

            last_c = c;
        }

        dp.last().map(|x| *x).unwrap_or(0)
    }

    fn decode_recursive(s: &[u8]) -> i32 {
        match (s.get(0), s.get(1)) {
            (Some(a), Some(b)) => {
                let x: i32 = String::from_utf8_lossy(&[*a, *b]).parse().unwrap();
                if *a != '0' as u8 {
                    Solution::decode_recursive(&s[1..]) + if x > 0 && x <= 26 {
                        Solution::decode_recursive(&s[2..])
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
            (Some(a), None) => {
                if *a != '0' as u8 {
                    1
                } else {
                    0
                }
            }
            _ => 1,
        }
    }

}

fn main() {
    let inputs = [
        "12",
        "226",
        "06",
        "27",
        "123123",
        "111111111111111111111111111111",
        "111111111111111111111111111111111111111111111",
    ];

    for input in inputs {
        println!("{}\t {}", input, Solution::num_decodings(input.to_string()));
    }
}
