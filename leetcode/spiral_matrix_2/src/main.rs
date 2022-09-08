struct Solution {}

impl Solution {
    pub fn generate_matrix(n: usize) -> Vec<Vec<i32>> {
        let mut mat = vec![vec![0; n]; n];

        let mut k = 1;
        for i in 0..n / 2 {
            for j in i..n - i {
                mat[i][j] = k;
                k += 1;
            }

            for j in i + 1..n - i {
                mat[j][n - 1 - i] = k;
                k += 1;
            }

            for j in (i..n - 1 - i).rev() {
                mat[n - 1 - i][j] = k;
                k += 1;
            }

            for j in (i + 1 .. n - i - 1).rev() {
                mat[j][i] = k;
                k += 1;
            }
        }

        if n % 2 == 1 {
            mat[n / 2][n / 2] = k;
        }

        mat
    }
}

fn main() {
    println!("{} -> {:?}", 3, Solution::generate_matrix(3));
    println!("{} -> {:?}", 4, Solution::generate_matrix(4));
    println!("{} -> {:?}", 5, Solution::generate_matrix(5));
}
