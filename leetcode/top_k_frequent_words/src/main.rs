use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

struct Solution {}

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut word_freq = HashMap::<String, i32>::new();
        for word in words.into_iter() {
            *word_freq.entry(word).or_default() += 1;
        }

        let mut heap = BinaryHeap::new();
        for (word, freq) in word_freq {
            heap.push((Reverse(freq), word));
            if heap.len() > (k as usize) {
                heap.pop();
            }
        }

        heap.into_sorted_vec().into_iter().map(|(_, word)| word).collect()
    }
}

fn main() {
    let inputs = [
        (vec!["i","love","leetcode","i","love","coding"], 2),
        (vec!["the","day","is","sunny","the","the","the","sunny","is","is"], 4)
    ];

    for (words, k) in inputs {
        println!("{:?}", Solution::top_k_frequent(words.into_iter().map(String::from).collect(), k));
    }
}
