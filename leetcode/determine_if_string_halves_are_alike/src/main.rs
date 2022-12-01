struct Solution {}

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        Self::count_vowels(&s[..s.len()/2]) == Self::count_vowels(&s[s.len()/2..])
    }

    fn count_vowels(s: &str) -> u32 {
        s.bytes().filter(Self::is_vowel).count() as u32
    }

    fn is_vowel(b: &u8) -> bool {
        for vowel in ['a', 'e', 'i', 'o', 'u'] {
            if vowel as u8 == b.to_ascii_lowercase() {
                return true
            }
        }

        false
    }
}

fn main() {
    println!("{}", Solution::halves_are_alike("book".to_string()));
    println!("{}", Solution::halves_are_alike("textbook".to_string()));
}
