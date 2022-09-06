use std::collections::BTreeSet;
use std::cmp;

struct Solution {}

impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (matrix.len(), matrix[0].len());
        let mut max_sum = std::i32::MIN;
        let mut curr_row = vec![0; m];

        for top in 0..n {
            for bot in top..n {
                for i in 0..m {
                    curr_row[i] += matrix[bot][i];
                }

                let sum = Solution::max_sub_array(&curr_row);
                if sum > max_sum {
                    max_sum = sum;
                }
            }

            curr_row.fill(0);
        }

        max_sum
    }

    pub fn max_sub_array(nums: &Vec<i32>) -> i32 {
        let (mut max_sum, mut curr_sum) = (i32::MIN, 0);

        for num in nums {
            curr_sum += num;

            if curr_sum > max_sum {
                max_sum = curr_sum
            }

            if curr_sum < 0 {
                curr_sum = 0;
            }
        }

        max_sum
    }

    pub fn max_sum_submatrix_less_than_k(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let (n, m) = (matrix.len(), matrix[0].len());
        let mut max_sum = std::i32::MIN;
        let mut curr_row = vec![0; m];

        for top in 0..n {
            for bot in top..n {
                for i in 0..m {
                    curr_row[i] += matrix[bot][i];
                }

                let sum = Solution::max_sub_array_less_than_k(&curr_row, k);
                if sum > max_sum {
                    if sum == k {
                        return k
                    }
                    max_sum = sum;
                }
            }

            curr_row.fill(0);
        }

        max_sum
    }

    pub fn max_sub_array_less_than_k(nums: &Vec<i32>, k: i32) -> i32 {
        let (mut max_sum, mut curr_sum) = (i32::MIN, 0);

        let mut prefix_sums = BTreeSet::new();
        prefix_sums.insert(0);

        for num in nums {
            curr_sum += num;

            let smallest_iter = prefix_sums.range(curr_sum - k..).next();
            if let Some(prefix) = smallest_iter {
                max_sum = cmp::max(max_sum, curr_sum - prefix);

                if max_sum == k {
                    return max_sum
                }
            }

            prefix_sums.insert(curr_sum);
        }

        max_sum
    }
}

fn main() {
    let inputs = vec![
        (vec![vec![1, 0, 1], vec![0, -2, 3]], 2),
        (vec![vec![2, 2, -1]], 3),
    ];

    for (matrix, k) in inputs {
        println!("{:?} -> {:?}", matrix.clone(), Solution::max_sum_submatrix(matrix.clone()));
        println!("{:?} {} -> {:?}", matrix.clone(), k, Solution::max_sum_submatrix_less_than_k(matrix, k));
    }
}
