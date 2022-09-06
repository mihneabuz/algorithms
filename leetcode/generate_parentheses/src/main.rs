struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        Self::build_solutions2(String::new(), n, n)
    }

    fn build_solutions1(acc: String, open: i32, closed: i32, solutions: &mut Vec<String>) {
        if open == 0 {
            let res = match closed == 0 {
                true  => acc,
                false => acc + ")".repeat(closed as usize).as_str(),
            };

            solutions.push(res);
            return
        }

        if open < closed {
            Self::build_solutions1(acc.clone() + "(", open - 1, closed, solutions);
            Self::build_solutions1(acc.clone() + ")", open, closed - 1, solutions);
        } else {
            Self::build_solutions1(acc + "(", open - 1, closed, solutions);
        }

    }

    fn build_solutions2(acc: String, open: i32, closed: i32) -> Vec<String> {
        if open == 0 {
            let res = match closed == 0 {
                true  => acc,
                false => acc + ")".repeat(closed as usize).as_str(),
            };

            return vec![res];
        }

        let mut solutions: Vec<String> = Vec::new();
        if open < closed {
            solutions.extend(Self::build_solutions2(acc.clone() + "(", open - 1, closed));
            solutions.extend(Self::build_solutions2(acc.clone() + ")", open, closed - 1));
        } else {
            solutions.extend(Self::build_solutions2(acc + "(", open - 1, closed));
        }

        solutions
    }
}

fn main() {
    println!("{} {:?}", 1, Solution::generate_parenthesis(1));
    println!("{} {:?}", 3, Solution::generate_parenthesis(3));
}
