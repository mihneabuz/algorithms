use std::cmp;

struct Solution {}

impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let (n, m) = (matrix.len(), matrix[0].len());

        for i in 1..n {
            let r = matrix[n - i][0];
            if !(1..cmp::min(i, m)).map(|j| matrix[n - i + j][j]).all(|x| x == r) {
                return false;
            }
        }

        for i in 0..m {
            let r = matrix[0][i];
            if !(1..cmp::min(n, m - i)).map(|j| matrix[j][i + j]).all(|x| x == r) {
                return false;
            };
        }

        true
    }
}

fn main() {
    let inputs = [
        vec![vec![1, 2, 3, 4], vec![5, 1, 2, 3], vec![9, 5, 1, 2]],
        vec![vec![1, 2], vec![2, 2]],
        vec![vec![83], vec![64], vec![57]],
    ];

    for input in inputs {
        println!("{}", Solution::is_toeplitz_matrix(input));
    }
}
