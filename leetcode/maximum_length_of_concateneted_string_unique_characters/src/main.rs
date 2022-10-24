use std::cmp;

struct Solution {}

impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let items = &arr
            .into_iter()
            .filter_map(Self::str_to_u32)
            .collect::<Vec<u32>>();

        let mut dp = vec![0u32];
        for x in items {
            for i in 0..dp.len() {
                if x & dp[i] == 0 {
                    dp.push(x | dp[i]);
                }
            }
        }

        dp.into_iter().map(|x| x.count_ones() as i32).max().unwrap_or(0)
    }

    pub fn max_length_recurse(arr: Vec<String>) -> i32 {
        Self::build_solution(
            0,
            &arr.into_iter()
                .filter_map(Self::str_to_u32)
                .collect::<Vec<u32>>(),
        )
    }

    fn str_to_u32(s: String) -> Option<u32> {
        let mut acc: u32 = 0;
        for b in s.bytes().map(|b| b - 'a' as u8) {
            if (acc & 1 << b) != 0 {
                return None;
            }

            acc |= 1 << b;
        }

        Some(acc)
    }

    fn build_solution(curr: u32, items: &[u32]) -> i32 {
        items.iter().fold(curr.count_ones() as i32, |best, x| {
            if curr & *x == 0 {
                cmp::max(best, Self::build_solution(curr | *x, items))
            } else {
                best
            }
        })
    }
}

fn main() {
    let inputs = [
        vec!["un", "iq", "ue"],
        vec!["cha", "r", "act", "ers"],
        vec!["abcdefghijklmnopqrstuvwxyz"],
        vec!["aa", "bb"],
    ];

    for input in inputs {
        println!(
            "{}",
            Solution::max_length(input.into_iter().map(String::from).collect())
        );
    }
}
