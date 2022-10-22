struct Solution {}

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let bytes = s.as_bytes();

        let mut target = vec![0i32; 60];
        for b in t.bytes().map(|b| b - 'A' as u8) {
            target[b as usize] += 1;
        }

        let mut current = vec![0i32; 60];
        let (mut i, mut j) = (0, 0);
        while !current.iter().zip(target.iter()).all(|(x, y)| *x >= *y) {
            if i >= bytes.len() {
                return "".to_string()
            }

            let b = bytes[i] - 'A' as u8;
            current[b as usize] += 1;
            i += 1;
        }

        let mut best = (i, j, i);
        loop {
            loop {
                let b = bytes[j] - 'A' as u8;
                if current[b as usize] - 1 < target[b as usize] {
                    break;
                }
                current[b as usize] -= 1;
                j += 1;
            }

            if i - j < best.0 {
                best = (i - j, j, i);
            }

            if i >= bytes.len() {
                break;
            }

            let b = bytes[i] - 'A' as u8;
            current[b as usize] += 1;
            i += 1;
        }

        return String::from_utf8_lossy(&bytes[best.1..best.2]).to_string()
    }
}

fn main() {
    let inputs = [
        ("ADOBECODEBANC", "ABC"),
        ("a", "a"),
        ("a", "aa"),
        ("aaaaaaaazAAAAAAAAZ", "azAZ")
    ];

    for (s, t) in inputs {
        println!("{}", Solution::min_window(s.to_string(), t.to_string()))
    }
}
