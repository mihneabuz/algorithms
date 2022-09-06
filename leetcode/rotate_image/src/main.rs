struct Solution {}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let (n, m) = (matrix.len(), matrix[0].len());

        if n != m {
            return;
        }

        for i in 0..n / 2 {
            for j in i..n - i - 1 {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[n - j - 1][i];
                matrix[n - j - 1][i] = matrix[n - i - 1][n - j - 1];
                matrix[n - i - 1][n - j - 1] = matrix[j][n - i - 1];
                matrix[j][n - i - 1] = temp;
            }
        }
    }
}

fn main() {
    let mut matrix2 = vec![vec![1,2],vec![3,4]];
    let mut matrix3 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let mut matrix4 = vec![
        vec![5, 1, 9, 11],
        vec![2, 4, 8, 10],
        vec![13, 3, 6, 7],
        vec![15, 14, 12, 16],
    ];

    println!("2x2");
    println!("{:?}", &matrix2);
    Solution::rotate(&mut matrix2);
    println!("{:?}", &matrix2);
    println!("3x3");
    println!("{:?}", &matrix3);
    Solution::rotate(&mut matrix3);
    println!("{:?}", &matrix3);
    println!("4x4");
    println!("{:?}", &matrix4);
    Solution::rotate(&mut matrix4);
    println!("{:?}", &matrix4);
}
