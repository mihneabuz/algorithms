struct Solution {}

type Tile = (bool, bool, bool);

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (n, m) = (heights.len(), heights[0].len());

        let mut visited = vec![vec![(false, false, false); m]; n];
        let mut results = Vec::new();
        for i in 0..n {
            for j in 0..m {
                let res = Solution::process(i as i32, j as i32, n as i32, m as i32, &heights, &mut visited);
                if res.1 && res.2 {
                    results.push(vec![i as i32, j as i32]);
                }
            }
        }

        for row in visited.iter() {
            println!("{:?}", row);
        }

        results
    }

    fn process(i: i32, j: i32, n: i32, m: i32, heights: &[Vec<i32>], visited: &mut [Vec<Tile>]) -> Tile {
        if i < 0 {
            (true, true, false)
        } else if j < 0 { 
            (true, true, false)
        } else if i >= n {
            (true, false, true)
        } else if j >= m { 
            (true, false, true)
        } else if visited[i as usize][j as usize].0 {
            visited[i as usize][j as usize]
        } else {
            visited[i as usize][j as usize] = (true, false, false);
            let result = [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)]
                .into_iter()
                .map(|(di, dj)| (i + di, j + dj))
                .filter(|(ni, nj)| heights[*ni as usize][*nj as usize] <= heights[i as usize][j as usize])
                .map(|(ni, nj)| Solution::process(ni, nj, n, m, heights, visited))
                .fold((true, false, false), |acc, res| (true, acc.1 || res.1, acc.2 || res.2));
            visited[i as usize][j as usize] = result;

            result
        }
    }

}

fn main() {
    let input = vec![
        vec![1, 2, 2, 3, 5],
        vec![3, 2, 3, 4, 4],
        vec![2, 4, 5, 3, 1],
        vec![6, 7, 1, 4, 5],
        vec![5, 1, 1, 2, 4],
    ];

    let res = Solution::pacific_atlantic(input);
    println!("{:?}", res);
}
