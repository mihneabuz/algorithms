struct Solution {}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();

        for token in tokens {
            if let Ok(num) = token.parse::<i32>() {
                stack.push(num);
            } else {
                let op2 = stack.pop().unwrap();
                let op1 = stack.pop().unwrap();

                stack.push(match token.as_str() {
                    "+" => op1 + op2,
                    "-" => op1 - op2,
                    "*" => op1 * op2,
                    "/" => op1 / op2,
                    _ => unreachable!()
                });
            }
        }

        stack.pop().unwrap()
    }
}

fn main() {
    println!("{}", Solution::eval_rpn(["2","1","+","3","*"].into_iter().map(String::from).collect()));
}
