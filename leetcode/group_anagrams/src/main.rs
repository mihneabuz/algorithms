use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<_, Vec<String>> = HashMap::new();

        for s in strs {
            groups.entry(Self::key_arr(&s)).or_default().push(s);
        }

        groups.into_values().collect()
    }

    fn key_str(str: &String) -> String {
        let mut copy = str.clone();
        unsafe {
            copy.as_bytes_mut().sort();
        }
        copy
    }

    fn key_arr(str: &String) -> [u8; 26] {
        str.bytes().map(|b| b - 'a' as u8).fold([0; 26], |mut acc, b| {
            acc[b as usize] += 1;
            acc
        })
    }

    fn key_u32(str: &String) -> u32 {
        str.bytes().map(|b| b - 'a' as u8).fold(0, |acc, b| acc | (1 << b))
    }
}

fn main() {
    let inputs = [
        vec!["eat","tea","tan","ate","nat","bat"],
        vec!["a"],
        vec![""],
    ];

    for input in inputs {
        println!("{:?}", Solution::group_anagrams(input.into_iter().map(String::from).collect()));
    }
}
