struct Solution {}

impl Solution {
    pub fn bag_of_tokens_score(mut tokens: Vec<i32>, mut power: i32) -> i32 {
        tokens.sort();

        let mut score = 0;
        let mut iter = tokens.into_iter();

        while let Some(token) = iter.next() {
            if token <= power {
                power -= token;
                score += 1;
            } else if score == 0 {
                break;
            } else if let Some(last) = iter.next_back() {
                if last > token {
                    power += last - token;
                }
            }
        }

        score
    }

    pub fn first_try(mut tokens: Vec<i32>, mut power: i32) -> i32 {
        let mut score = 0;
        tokens.sort();

        let mut i = 0;
        while i < tokens.len() {
            while i < tokens.len() && power >= tokens[i] {
                power -= tokens[i];
                score += 1;
                i += 1;
            }

            if i < tokens.len() {
                if *tokens.last().unwrap() > tokens[i] && score > 0 {
                    power += tokens.pop().unwrap();
                    score -= 1;
                } else {
                    break;
                }
            }
        }

        score
    }
}

fn main() {
    let inputs = vec![
        (vec![100], 50),
        (vec![100, 200], 150),
        (vec![100, 200, 300, 400], 200),
        (vec![71, 55, 82], 54),
    ];

    for (tokens, power) in inputs {
        println!("{}", Solution::bag_of_tokens_score(tokens, power));
    }
}
