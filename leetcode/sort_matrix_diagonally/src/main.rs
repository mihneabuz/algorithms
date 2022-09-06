struct Solution {}

use std::cmp;

impl Solution {
    pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if mat.is_empty() {
            return vec![];
        }

        if mat.len() == 1 || mat[0].len() == 1 {
            return mat;
        }

        let (n, m) = (mat.len(), mat[0].len());
        let mut res = vec![vec![0; m]; n];
        let mut diag = Vec::with_capacity(cmp::min(n, m));

        for i in 0..n {
            for k in 0..cmp::min(n - i, m) {
                diag.push(mat[i + k][k]);
            }

            diag.sort();

            for k in (0..cmp::min(n - i, m)).rev() {
                res[i + k][k] = diag.pop().unwrap();
            }
        }

        for j in 1..m {
            for k in 0..cmp::min(n, m - j) {
                diag.push(mat[k][j + k]);
            }

            diag.sort();

            for k in (0..cmp::min(n, m - j)).rev() {
                res[k][j + k] = diag.pop().unwrap();
            }
        }

        res
    }
}

fn main() {
    let tests = [
        (
            vec![vec![3, 3, 1, 1], vec![2, 2, 1, 2], vec![1, 1, 1, 2]],
            vec![vec![1, 1, 1, 1], vec![1, 2, 2, 2], vec![1, 2, 3, 3]],
        ),
        (
            vec![
                vec![11, 25, 66, 1, 69, 7],
                vec![23, 55, 17, 45, 15, 52],
                vec![75, 31, 36, 44, 58, 8],
                vec![22, 27, 33, 25, 68, 4],
                vec![84, 28, 14, 11, 5, 50],
            ],
            vec![
                vec![5, 17, 4, 1, 52, 7],
                vec![11, 11, 25, 45, 8, 69],
                vec![14, 23, 25, 44, 58, 15],
                vec![22, 27, 31, 36, 50, 66],
                vec![84, 28, 75, 33, 55, 68],
            ],
        ),
        (
            vec![vec![75, 21, 13, 24, 8], vec![24, 100, 40, 49, 62]],
            vec![vec![75, 21, 13, 24, 8], vec![24, 100, 40, 49, 62]],
        ),
    ];

    for (input, output) in tests {
        println!("-> {}", Solution::diagonal_sort(input).eq(&output));
    }
}
