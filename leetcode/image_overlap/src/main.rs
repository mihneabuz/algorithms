use std::cmp::{min, max};

struct Solution {}

impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        Self::possible_translations(img1.len() as i32)
            .into_iter()
            .map(|t| Self::overlap(&img1, &img2, t))
            .max()
            .unwrap_or(0)
    }

    pub fn overlap(img1: &[Vec<i32>], img2: &[Vec<i32>], t: (i32, i32)) -> i32 {
        let mut count = 0;
        let n = img1.len() as i32;

        for i in max(0, t.0)..min(n, n + t.0) {
            for j in max(0, t.1)..min(n, n + t.1) {
                let (ti, tj) = (i - t.0, j - t.1);
                count += img1[ti as usize][tj as usize] & img2[i as usize][j as usize];
            }
        }

        count
    }

    pub fn possible_translations(n: i32) -> Vec<(i32, i32)> {
        let mut res = Vec::with_capacity(n as usize * n as usize);

        for i in 1 - n..n {
            for j in 1 - n..n {
                res.push((i, j));
            }
        }

        return res;
    }
}

fn main() {
    let inputs = [
        (
            vec![vec![1, 1, 0], vec![0, 1, 0], vec![0, 1, 0]],
            vec![vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 1]],
        ),
        (vec![vec![1]], vec![vec![1]]),
        (vec![vec![0]], vec![vec![0]]),
    ];

    for (img1, img2) in inputs {
        println!("{}", Solution::largest_overlap(img1, img2));
    }
}
