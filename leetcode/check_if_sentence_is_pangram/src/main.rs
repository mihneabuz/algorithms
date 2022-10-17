struct Solution {}

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut letters = vec![0; 26];

        sentence.bytes().map(|byte| byte - 'a' as u8).for_each(|idx| {
            letters[idx as usize] += 1;
        });

        letters.into_iter().all(|x| x > 0)
    }
}

fn main() {
    let inputs = [
        "thequickbrownfoxjumpsoverthelazydog",
        "anaaremere"
    ];

    for input in inputs {
        println!("{}", Solution::check_if_pangram(String::from(input)));
    }
}
