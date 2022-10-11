struct Solution {}

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let (mut first, mut second) = (nums[0], None);

        for num in nums[1..].into_iter() {
            if second.is_some() && num > second.unwrap() {
                return true;
            }

            if *num <= first {
                first = *num;
            } else {
                second = Some(num);
            }
        }

        false
    }
}

fn main() {
    let inputs = [
        vec![1, 2, 3, 4, 5],
        vec![5, 4, 3, 2, 1],
        vec![2, 1, 5, 0, 4, 6],
        vec![1, 1, -2, 6],
    ];

    for input in inputs {
        println!("{:?}", Solution::increasing_triplet(input));
    }
}
