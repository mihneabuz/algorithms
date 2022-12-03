use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut freq = HashMap::new();
        for b in s.bytes() {
            freq.entry(b).and_modify(|f| *f += 1).or_insert(1);
        }

        let mut chars: Vec<(u8, i32)> = freq.into_iter().collect();
        chars.sort_by_key(|c| -c.1);

        chars
            .into_iter()
            .map(|c| String::from(c.0 as char).repeat(c.1 as usize))
            .collect::<String>()
    }
}

fn main() {
    println!("{}", Solution::frequency_sort("tree".to_string()));
}
