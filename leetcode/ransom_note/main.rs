struct Solution {}

use std::collections::HashMap;

// trying out hashmap, this would be much easier with a simple array
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut set = HashMap::with_capacity(256);

        for c in magazine.chars() {
            set.entry(c).and_modify(|count| *count += 1).or_insert(1);
        }

        for c in ransom_note.chars() {
            let count = set.entry(c).and_modify(|count| *count -= 1).or_insert(-1);

            if *count < 0 {
                return false;
            }
        }

        true
    }
}

fn main() {
    let tests = vec![
        ("a", "b", false),
        ("aa", "ab", false),
        ("aa", "aab", true),
    ];

    for (note, magazine, expected) in tests {
        let result = Solution::can_construct(String::from(note), String::from(magazine));
        println!("{} {} -> {} {}", note, magazine, result, expected);
    }
}
