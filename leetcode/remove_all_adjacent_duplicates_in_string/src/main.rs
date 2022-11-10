struct Solution {}

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack = Vec::new();

        for b in s.into_bytes() {
            if stack.last().map(|&top| top == b).unwrap_or(false) {
                stack.pop();
            } else {
                stack.push(b);
            }
        }

        String::from_utf8_lossy(&stack).to_string()
    }
}

fn main() {
    println!("{}", Solution::remove_duplicates("abbaca".to_string()));
}
