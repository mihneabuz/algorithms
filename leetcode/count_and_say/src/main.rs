struct Solution {}

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut s = vec!['1' as u8];

        for _ in 1..n {
            let mut iter = s.into_iter().peekable();
            let mut s2 = Vec::new();

            while let Some(byte) = iter.next() {
                let mut count = 1;
                while iter.peek().map(|next| *next == byte).unwrap_or(false) {
                    iter.next();
                    count += 1;
                }

                count.to_string().bytes().for_each(|b| s2.push(b));
                s2.push(byte);
            }

            s = s2;
        }


        String::from_utf8_lossy(&s).to_string()
    }
}

fn main() {
    let inputs = [1, 4];

    for input in inputs {
        println!("{}", Solution::count_and_say(input));
    }
}
