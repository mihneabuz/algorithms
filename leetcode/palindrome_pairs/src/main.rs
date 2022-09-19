use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let word_map: HashMap<&str, usize> = words
            .iter()
            .enumerate()
            .map(|(i, w)| (w.as_str(), i))
            .collect();

        words.iter().enumerate().fold(Vec::new(), |mut acc, (i, word)| {
            let n = word.len();
            let rev: String = word.chars().rev().collect();

            let from_right = (0..n)
                .filter(|&k| word[..n - k].chars().eq(word[..n - k].chars().rev()))
                .filter_map(|k| word_map.get(&rev[..k]).map(|&j| (j, i)));

            let from_left = (0..=n)
                .filter(|&k| word[k..].chars().eq(word[k..].chars().rev()))
                .filter_map(|k| word_map.get(&rev[n - k..]).map(|&j| (i, j)));

            from_left.chain(from_right)
                .filter(|&(i, j)| i != j)
                .for_each(|(i, j)| acc.push(vec![i as i32, j as i32]));

            acc
        })
    }

    pub fn palindrome_pairs_old(words: Vec<String>) -> Vec<Vec<i32>> {
        let map: HashMap<&str, i32> = words
            .iter()
            .enumerate()
            .map(|(i, s)| (s.as_str(), i as i32))
            .collect();

        let empty_index = map.get("").map(|i| *i).unwrap_or(-1);

        words.iter().enumerate().fold(Vec::new(), |mut res, (k, word)| {
            if Solution::is_palindrome(word) {
                if empty_index >= 0 && empty_index != k as i32 {
                    res.push(vec![k as i32, empty_index]);
                    res.push(vec![empty_index, k as i32]);
                }
            } else {
                if let Some(pair) = map.get(word.chars().rev().collect::<String>().as_str()) {
                    res.push(vec![k as i32, *pair]);
                }
            }

            for i in 1..word.len() {
                if Solution::is_palindrome(&word[i..]) {
                    if let Some(pair) = map.get(word[..i].chars().rev().collect::<String>().as_str()) {
                        res.push(vec![k as i32, *pair]);
                    }
                }

                if Solution::is_palindrome(&word[..i]) {
                    if let Some(pair) = map.get(word[i..].chars().rev().collect::<String>().as_str()) {
                        res.push(vec![*pair, k as i32]);
                    }
                }
            }

            res
        })
    }

    fn is_palindrome(slice: &str) -> bool {
        let mut iter = slice.bytes();
        while let Some(left) = iter.next() {
            if let Some(right) = iter.next_back() {
                if left != right {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {
    let inputs = [
        vec!["abcd", "dcba", "lls", "s", "sssll"],
        vec!["bat", "tab", "cat"],
        vec!["", "abcba", "abb"],
        vec!["a", "b", "c", "ab", "ac", "aa"],
    ];

    for input in inputs {
        println!("{:?}", Solution::palindrome_pairs(input.into_iter().map(String::from).collect()));
    }
}
