use std::cmp;

struct Solution {}

impl Solution {
    pub fn min_falling_path_sum(mut matrix: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (matrix.len(), matrix[0].len());

        for i in (0..n-1).rev() {
            matrix[i][0] += cmp::min(matrix[i + 1][0], matrix[i + 1][1]);
            for j in 1..m-1 {
                matrix[i][j] += cmp::min(matrix[i + 1][j], cmp::min(matrix[i + 1][j - 1], matrix[i + 1][j + 1]));
            }
            matrix[i][m - 1] += cmp::min(matrix[i + 1][m - 1], matrix[i + 1][m - 2]);
        }

        matrix.into_iter().next().unwrap().into_iter().min().unwrap()
    }
}

fn main() {}
