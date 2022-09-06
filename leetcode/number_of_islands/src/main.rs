struct Solution {}

static GROUND: char = '1';
static WATER: char = '0';

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut count = 0;
        let (n, m) = (grid.len(), grid[0].len());

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == GROUND {
                    Solution::fill_island(&mut grid, i, j, n as i32, m as i32);
                    count += 1;
                }
            }
        }

        count
    }

    fn fill_island(grid: &mut [Vec<char>], i: usize, j: usize, n: i32, m: i32) {
        let mut coords = vec![(i, j)];
        while !coords.is_empty() {
            let (i, j) = coords.pop().unwrap();
            grid[i][j] = WATER;
            coords.extend(Solution::neighbors(i, j, n, m).filter(|(i, j)| grid[*i][*j] == GROUND));
        }
    }

    fn is_valid(i: i32, j: i32, n: i32, m: i32) -> bool {
        i >= 0 && i < n && j >= 0 && j < m
    }

    fn neighbors(i: usize, j: usize, n: i32, m: i32) -> impl Iterator<Item = (usize, usize)> {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .map(move |(di, dj)| (di + i as i32, dj + j as i32))
            .filter(move |(i, j)| Solution::is_valid(*i, *j, n, m))
            .map(move |(i, j)| (i as usize, j as usize))
    }
}

fn main() {
    let grid1 = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];

    let grid2 = vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1'],
    ];

    println!("grid1: {}", Solution::num_islands(grid1));
    println!("grid1: {}", Solution::num_islands(grid2));
}
