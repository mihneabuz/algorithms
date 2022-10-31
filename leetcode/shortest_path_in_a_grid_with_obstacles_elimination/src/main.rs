use std::collections::VecDeque;
use std::cmp;

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

struct Solution {}

impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        Self::shortest_path_djikstra(grid, k)
    }

    fn shortest_path_djikstra(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let (n, m) = (grid.len(), grid[0].len());

        let mut visited = vec![vec![-1; m]; n];
        visited[0][0] = k;

        let mut queue = VecDeque::new();
        queue.push_back((cmp::Reverse(0), k, 0, 0));

        while let Some((cmp::Reverse(path), k, i, j)) = queue.pop_front() {
            if i == n - 1 && j == m - 1 {
                return path;
            }

            DIRECTIONS
                .iter()
                .filter_map(|dir| Self::neigs_in_bounds(i, j, dir, n, m))
                .for_each(|(di, dj)| {
                    let (di, dj) = (di as usize, dj as usize);

                    let next_k = k - grid[di][dj];
                    if next_k < 0 {
                        return;
                    }

                    if visited[di][dj] >= next_k {
                        return;
                    }
                    visited[di][dj] = next_k;

                    queue.push_back((cmp::Reverse(path + 1), next_k, di, dj));
                });
        }

        -1
    }

    fn shortest_path_bfs(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let (n, m) = (grid.len(), grid[0].len());

        let mut dist = vec![vec![-1; m]; n];
        let mut queue = VecDeque::new();

        dist[n - 1][m - 1] = 0;
        queue.push_back((n - 1, m - 1));

        for iter in 0..=k {
            while let Some((x, y)) = queue.pop_front() {
                DIRECTIONS
                    .iter()
                    .filter_map(|dir| Self::neigs_in_bounds(x, y, dir, n, m))
                    .for_each(|(dx, dy)| {
                        let (dx, dy) = (dx as usize, dy as usize);
                        if grid[dx][dy] != 1 && (dist[dx][dy] == -1 || dist[dx][dy] > 1 + dist[x][y]) {
                            dist[dx][dy] = 1 + dist[x][y];
                            queue.push_back((dx, dy));
                        }
                    });
            }

            if iter == k {
                break;
            }

            for (x, row) in grid.iter().enumerate() {
                for (y, cell) in row.iter().enumerate() {
                    if *cell == 1 {
                        let maybe_min = DIRECTIONS
                            .iter()
                            .filter_map(|dir| Self::neigs_in_bounds(x, y, dir, n, m))
                            .filter_map(|(dx, dy)| match dist[dx as usize][dy as usize] {
                                -1 => None,
                                d @ _ => Some(d),
                            })
                            .min();

                        if let Some(min) = maybe_min {
                            if dist[x][y] == -1 || dist[x][y] > min + 1 {
                                dist[x][y] = min + 1;
                                queue.push_back((x, y));
                            }
                        }
                    }
                }
            }
        }

        dist[0][0]
    }

    fn neigs_in_bounds(x: usize, y: usize, dir: &(i32, i32), n: usize, m: usize) -> Option<(i32, i32)> {
        let (x, y) = (dir.0 + x as i32, dir.1 + y as i32);
        if x >= 0 && x < (n as i32) && y >= 0 && y < (m as i32) {
            Some((x, y))
        } else {
            None
        }
    }
}

fn main() {
    let inputs = [
        (
            vec![
                vec![0, 0, 0],
                vec![1, 1, 0],
                vec![0, 0, 0],
                vec![0, 1, 1],
                vec![0, 0, 0],
            ],
            2,
        ),
        (vec![vec![0, 1, 1], vec![1, 1, 1], vec![1, 0, 0]], 1),
        (
            vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 0],
                vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 1, 1, 1, 1, 1, 1, 1],
                vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 0],
                vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 1, 1, 1, 1, 1, 1, 1],
                vec![0, 1, 0, 1, 1, 1, 1, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0, 0, 1, 0],
                vec![0, 1, 1, 1, 1, 1, 1, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
            ],
            1,
        ),
    ];

    for (grid, k) in inputs {
        println!("{}", Solution::shortest_path(grid, k));
    }
}
