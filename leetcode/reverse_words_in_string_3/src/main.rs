struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split(' ')
            .map(|w| w.chars().rev().collect::<String>())
            .reduce(|acc, w| acc + " " + &w)
            .unwrap_or(String::from(""))
    }
}

fn main() {
    let inputs = [
        "Let's take LeetCode contest",
        "God Ding"
    ];

    for input in inputs {
        println!("{}", Solution::reverse_words(String::from(input)));
    }
}
