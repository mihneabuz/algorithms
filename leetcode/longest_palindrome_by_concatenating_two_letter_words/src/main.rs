use std::collections::{HashMap, HashSet};

struct Solution {}

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut pair_map: HashMap<[u8; 2], i32> = HashMap::new();
        let mut symetric: HashSet<[u8; 2]> = HashSet::new();

        let mut pairs = 0;
        for word in words {
            if let [b0, b1] = *word.as_bytes() {
                if let Some(count) = pair_map.get_mut(&[b1, b0]) {
                    if *count > 0 {
                        *count -= 1;
                        pairs += 1;
                        if b0 == b1 && *count == 0 {
                            symetric.remove(&[b0, b1]);
                        }
                        continue;
                    }
                }

                pair_map.entry([b0, b1]).and_modify(|c| *c += 1).or_insert(1);

                if b0 == b1 {
                    symetric.insert([b0, b1]);
                }
            }
        }

        4 * pairs + 2 * !symetric.is_empty() as i32
    }
}

fn main() {
    let inputs = [
        vec!["lc", "cl", "gg"],
        vec!["ab", "ty", "yt", "lc", "cl", "ab"],
        vec!["cc", "ll", "xx"],
        vec!["dd", "aa", "bb", "dd", "aa", "dd", "bb", "dd", "aa", "cc", "bb", "cc", "dd", "cc"],
        vec!["bb", "bb"],
    ];

    for input in inputs {
        println!(
            "{}",
            Solution::longest_palindrome(input.into_iter().map(String::from).collect())
        );
    }
}
