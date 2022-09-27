struct Solution {}

const EMPTY: u8 = '.' as u8;
const LEFT: u8 = 'L' as u8;
const RIGHT: u8 = 'R' as u8;

impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        if dominoes.len() < 2 {
            return dominoes;
        }

        let mut bytes: Vec<u8> = dominoes.into_bytes();
        let mut next: Vec<u8> = bytes.clone();

        let mut done = false;
        while !done {
            done = true;

            for (i, next) in next.iter_mut().enumerate() {
                *next = match (i.checked_sub(1).map(|i| &bytes[i]), bytes.get(i), bytes.get(i + 1)) {
                    (Some(&RIGHT), Some(&EMPTY), Some(&LEFT)) => EMPTY,

                    (Some(&RIGHT), Some(&EMPTY), _) => {
                        done = false;
                        RIGHT
                    }

                    (_, Some(&EMPTY), Some(&LEFT)) => {
                        done = false;
                        LEFT
                    }

                    _ => bytes[i],
                }
            }

            bytes = next.clone();
        }

        String::from_utf8(bytes).unwrap()
    }
}

fn main() {
    let inputs = ["RR.L", ".L.R...LR..L.."];

    for input in inputs {
        println!("{}", Solution::push_dominoes(input.to_string()));
    }
}
