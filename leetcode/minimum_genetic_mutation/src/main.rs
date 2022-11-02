struct Solution {}

impl Solution {
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        let (start, end) = (Self::convert(start), Self::convert(end));
        let bank: Vec<[u8; 8]> = bank.into_iter().map(Self::convert).collect();

        let mut mutations = vec![&start];
        for iter in 0..=bank.len() {
            let mut next_mutations = Vec::new();

            for current in mutations {
                if *current == end {
                    return iter as i32;
                }

                for possible in bank.iter() {
                    if (*current).into_iter().zip(*possible).filter(|(b1, b2)| *b1 != *b2).count() == 1 {
                        next_mutations.push(possible);
                    }
                }
            }

            mutations = next_mutations;
        }

        -1
    }

    fn convert(s: String) -> [u8; 8] {
        let mut acc = [0; 8];
        for (i, b) in s.bytes().enumerate() {
            acc[i] = b - 'A' as u8;
        }
        acc
    }
}

fn main() {
    let inputs = [
        ("AACCGGTT", "AACCGGTA", vec!["AACCGGTA"]),
        ("AACCGGTT", "AAACGGTA", vec!["AACCGGTA","AACCGCTA","AAACGGTA"]),
        ("AAAAACCC", "AACCCCCC", vec!["AAAACCCC","AAACCCCC","AACCCCCC"]),
        ("AACCGGTT", "AAACGGTA", vec!["AACCGATT","AACCGATA","AAACGATA","AAACGGTA"]),
    ];

    for (start, end, bank) in inputs {
        println!("{}\n", Solution::min_mutation(
            start.to_string(),
            end.to_string(),
            bank.into_iter().map(String::from).collect())
        );
    }
}
