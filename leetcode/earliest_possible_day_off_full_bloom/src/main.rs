use std::cmp;

struct Solution {}

impl Solution {
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        let mut flowers: Vec<(i32, i32)> = plant_time.into_iter().zip(grow_time.into_iter()).collect();
        flowers.sort_by_key(|f| cmp::Reverse(f.1));

        let (mut sum, mut res) = (0, 0);
        for (plant, grow) in flowers.into_iter() {
            sum += plant;
            res = cmp::max(res, sum + grow);
        }

        res
    }
}

fn main() {
    let inputs = [
        (vec![1, 4, 3], vec![2, 3, 1]),
        (vec![1, 2, 3, 2], vec![2, 1, 2, 1]),
        (
            vec![24, 28,  9,  1,  9, 27, 10, 10,  1,  4, 29, 29],
            vec![15, 15, 21, 19, 22, 27,  3, 18, 19, 16, 15, 22],
        ),
        (
            vec![27, 5, 24, 17, 27, 4, 23, 16, 6, 26, 13, 17, 21, 3, 9, 10, 28, 26, 4, 10, 28, 2],
            vec![26, 9, 14, 17, 6, 14, 23, 24, 11, 6, 27, 14, 13, 1, 15, 5, 12, 15, 23, 27, 28, 12],
        ),
    ];

    for (p, g) in inputs {
        println!("{}", Solution::earliest_full_bloom(p, g));
    }
}
