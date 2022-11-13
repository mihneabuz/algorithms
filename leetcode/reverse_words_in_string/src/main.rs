#![feature(iter_intersperse)]

struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
    }

    pub fn reverse_words_1(s: String) -> String {
        String::from_utf8(
            s.trim()
                .as_bytes()
                .split(|&b| b == ' ' as u8)
                .filter(|w| w.len() > 0)
                .rev()
                .fold(Vec::new(), |mut acc, word| {
                    if acc.len() > 0 { acc.push(' ' as u8) }
                    acc.extend_from_slice(word);
                    acc
                }),
        )
        .unwrap()
    }

    pub fn reverse_words_2(s: String) -> String {
        String::from_utf8(
            s.trim()
                .as_bytes()
                .split(|&b| b == ' ' as u8)
                .filter(|w| w.len() > 0)
                .rev()
                .intersperse(&[' ' as u8])
                .flatten()
                .map(|&b| b)
                .collect(),
        )
        .unwrap()
    }
}

fn main() {
    println!("{}", Solution::reverse_words("  hello world  ".to_string()));
    println!("{}", Solution::reverse_words("a good   example".to_string()));
}
