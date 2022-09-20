struct Solution {}

impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (n, m) = (nums1.len(), nums2.len());
        let mut dp = vec![vec![0; m]; n];

        (0..n).for_each(|i| { dp[i][0] = (nums1[i] == nums2[0]) as i32 });
        (0..m).for_each(|j| { dp[0][j] = (nums1[0] == nums2[j]) as i32 });

        for (i, num1) in nums1.iter().enumerate().skip(1) {
            for (j, num2) in nums2.iter().enumerate().skip(1) {
                if *num1 == *num2 {
                    dp[i][j] = 1 + dp[i - 1][j - 1];
                }
            }
        }

        dp.into_iter().flatten().max().unwrap_or(0)
    }
}

/*
*
*    X  1  2  3  2  1
*    3  0  0  1  0  0
*    2  0  1  0  2  0
*    1  1  0  0  0  3
*    4  0  0  0  0  0
*    7  0  0  0  0  0
*
*/

fn main() {
    let inputs = [
        (vec![1,2,3,2,1], vec![3,2,1,4,7]),
        (vec![0,0,0,0,0], vec![0,0,0,0,0])
    ];

    for (nums1, nums2) in inputs {
        println!("{}", Solution::find_length(nums1, nums2));
    }
}
