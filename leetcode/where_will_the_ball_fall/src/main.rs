use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        (0..grid[0].len()).map(|i| Self::fall(&grid, 0, i)).collect()
    }

    pub fn cached_fall(grid: &[Vec<i32>], x: usize, y: usize, cache: &mut HashMap<(usize, usize), i32>) -> i32 {
        if x == grid.len() {
            return y as i32;
        }

        if let Some(res) = cache.get(&(x, y)) {
            return *res;
        }

        let res = match grid[x][y] {
            1 => {
                if y + 1 == grid[x].len() || grid[x][y + 1] == -1 {
                    -1
                } else {
                    Self::cached_fall(grid, x + 1, y + 1, cache)
                }
            },
            -1 => {
                if y == 0 || grid[x][y - 1] == 1 {
                    -1
                } else {
                    Self::cached_fall(grid, x + 1, y - 1, cache)
                }
            },
            _ => unreachable!()
        };

        cache.insert((x, y), res);

        res
    }

    pub fn fall(grid: &[Vec<i32>], x: usize, y: usize) -> i32 {
        if x == grid.len() {
            return y as i32;
        }

        match grid[x][y] {
            1 => {
                if y + 1 == grid[x].len() || grid[x][y + 1] == -1 {
                    -1
                } else {
                    Self::fall(grid, x + 1, y + 1)
                }
            },
            -1 => {
                if y == 0 || grid[x][y - 1] == 1 {
                    -1
                } else {
                    Self::fall(grid, x + 1, y - 1)
                }
            },
            _ => unreachable!()
        }
    }
}

fn main() {
    let inputs = [
        vec![vec![1,1,1,-1,-1],vec![1,1,1,-1,-1],vec![-1,-1,-1,1,1],vec![1,1,1,1,-1],vec![-1,-1,-1,-1,-1]],
        vec![vec![-1]]
    ];

    for input in inputs {
        println!("{:?}", Solution::find_ball(input));
    }
}
