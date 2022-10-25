struct Solution {}

impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        if word1.iter().map(|s| s.len()).sum::<usize>() != word2.iter().map(|s| s.len()).sum::<usize>() {
            return false;
        }

        word1
            .into_iter()
            .flat_map(|s| s.into_bytes())
            .zip(word2.into_iter().flat_map(|s| s.into_bytes()))
            .all(|(a, b)| a == b)
    }
}

fn main() {
    let inputs = [
        (vec!["ab", "c"], vec!["a", "bc"]),
        (vec!["a", "cb"], vec!["ab", "c"]),
    ];

    for (word1, word2) in inputs {
        println!(
            "{}",
            Solution::array_strings_are_equal(
                word1.into_iter().map(String::from).collect(),
                word2.into_iter().map(String::from).collect()
            )
        );
    }
}
