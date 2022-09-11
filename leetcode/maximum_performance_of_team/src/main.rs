use std::cmp;
use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        let mut engineers: Vec<(i32, i32)> = speed.into_iter().zip(efficiency.into_iter()).collect();
        engineers.sort_by_key(|eng| -eng.1);

        let (mut sum, mut res) = (0u64, 0u64);
        let mut heap = BinaryHeap::new();

        for (i, (speed, eff)) in engineers.into_iter().map(|(s, e)| (s as u64, e as u64)).enumerate() {
            heap.push(cmp::Reverse(speed));
            sum += if i < k as usize { speed } else { speed - heap.pop().unwrap().0 };
            res = cmp::max(res, sum * eff);
        }

        (res % (10_u64.pow(9) + 7)) as i32
    }

    fn dp_solution(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;

        let mut sums = vec![0u64; k + 1];
        let mut mins = vec![u64::MAX; k + 1];

        for i in 0..n as usize {
            for k in (1..k + 1).rev() {
                let prev = sums[k] * mins[k];
                let curr = (sums[k - 1] + speed[i] as u64) * cmp::min(mins[k - 1], efficiency[i] as u64);

                if curr > prev {
                    sums[k] = sums[k - 1] + speed[i] as u64;
                    mins[k] = cmp::min(mins[k - 1], efficiency[i] as u64);
                }
            }
        }

        let mut best = 0;
        for k in 1..k + 1 {
            best = cmp::max(best, sums[k] * mins[k]);
        }

        (best % (10u64.pow(9) + 7)) as i32
    }
}

fn main() {
    let inputs = vec![
        (6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 2),
        (6, vec![2, 3, 1, 5, 8, 10], vec![5, 3, 9, 7, 2, 4], 2),
        (6, vec![10, 2, 3, 1, 5, 8], vec![4, 5, 3, 9, 7, 2], 2),
        (6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 3),
        (3, vec![2, 8, 2], vec![2, 7, 1], 2),
    ];

    for (n, speed, efficiency, k) in inputs {
        println!("-> {}", Solution::max_performance(n, speed, efficiency, k));
    }
}
