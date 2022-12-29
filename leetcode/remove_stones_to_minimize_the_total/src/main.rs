use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::from(piles);

        for _ in 0..k {
            let mut val = heap.pop().unwrap();
            val = val - val / 2;
            heap.push(val);
        }

        heap.into_iter().sum()
    }
}

fn main() {
    println!("{}", Solution::min_stone_sum(vec![5, 4, 9], 2));
    println!(
        "{}",
        Solution::min_stone_sum(
            vec![8916, 7289, 8226, 4395, 589, 450, 5965, 7617, 5218, 6227],
            7
        )
    );
}
