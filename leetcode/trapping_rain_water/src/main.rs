use std::cmp;

struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut res = 0;
        let (mut max_left, mut max_right) = (0, 0);
        let (mut left, mut right) = (0, height.len() - 1);

        while left < right {
            let (height_left, height_right) = (height[left], height[right]);

            if height_left < height_right {
                max_left = cmp::max(max_left, height_left);
                res += max_left - height_left;
                left += 1;
            } else {
                max_right = cmp::max(max_right, height_right);
                res += max_right - height_right;
                right -= 1;
            }
        }

        res
    }
}


fn main() {
    let inputs = vec![
        vec![0,1,0,2,1,0,1,3,2,1,2,1],
        vec![4,2,0,3,2,5]
    ];

    for input in inputs {
        println!("{}", Solution::trap(input));
    }
}
