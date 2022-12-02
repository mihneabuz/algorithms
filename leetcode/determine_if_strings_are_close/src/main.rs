use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false
        }

        let (w1, w2) = (word1.as_bytes(), word2.as_bytes());
        let (mut freq1, mut freq2) = (vec![0; 26], vec![0; 26]);
        for i in 0..w1.len() {
            freq1[(w1[i] - 'a' as u8) as usize] += 1;
            freq2[(w2[i] - 'a' as u8) as usize] += 1;
        }

        let mut map = HashMap::new();
        for (freq1, freq2) in freq1.into_iter().zip(freq2.into_iter())  {
            if (freq1 == 0 && freq2 > 0) || (freq1 > 0 && freq2 == 0) {
                return false
            }

            if freq1 > 0 {
                map.entry(freq1).and_modify(|f| *f += 1).or_insert(1);
            }
            if freq2 > 0 {
                map.entry(freq2).and_modify(|f| *f -= 1).or_insert(-1);
            }
        }

        map.values().all(|&freq| freq == 0)
    }
}

fn main() {
    println!("{}", Solution::close_strings("abc".to_string(), "bca".to_string()));
    println!("{}", Solution::close_strings("a".to_string(), "aa".to_string()));
    println!("{}", Solution::close_strings("cabbba".to_string(), "abbccc".to_string()));
}
