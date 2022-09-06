struct Solution {}

#[derive(Clone, Debug)]
struct Tile {
    height: i32,
    pacific: bool,
    atlantic: bool,
}

impl Tile {
    pub fn new(height: i32) -> Self {
        Tile {
            height,
            pacific: false,
            atlantic: false,
        }
    }

    pub fn combine_oceans(from: &Tile, to: &mut Tile) -> bool {
        let res = (from.atlantic && !to.atlantic) || (from.pacific && !to.pacific);

        to.atlantic |= from.atlantic;
        to.pacific |= from.pacific;

        res
    }

    pub fn both(&self) -> bool {
        self.pacific && self.atlantic
    }

    pub fn set(&mut self, pacific: bool, atlantic: bool) {
        self.pacific = pacific;
        self.atlantic = atlantic;
    }
}

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (n, m) = (heights.len(), heights[0].len());

        if m == 1 {
            return (0..n).map(|i| vec![i as i32, 0]).collect();
        }

        if n == 1 {
            return (0..m).map(|j| vec![0, j as i32]).collect();
        }

        let mut tiles = Self::init_tiles(n, m, heights);

        for i in 0..n {
            Self::expand_tile(i, 0, n, m, &mut tiles);
            Self::expand_tile(i, m - 1, n, m, &mut tiles);
        }

        for j in 0..m {
            Self::expand_tile(0, j, n, m, &mut tiles);
            Self::expand_tile(n - 1, j, n, m, &mut tiles);
        }

        let mut results = Vec::new();
        for i in 0..n {
            for j in 0..m {
                if tiles[i][j].both() {
                    results.push(vec![i as i32, j as i32]);
                }
            }
        }

        results
    }

    fn init_tiles(n: usize, m: usize, heights: Vec<Vec<i32>>) -> Vec<Vec<Tile>> {
        let mut tiles: Vec<Vec<Tile>> = heights
            .into_iter()
            .map(|row| row.into_iter().map(|height| Tile::new(height)).collect())
            .collect();

        for i in 0..n {
            tiles[i][0].set(true, false);
            tiles[i][m - 1].set(false, true);
        }

        for j in 0..m {
            tiles[0][j].set(true, false);
            tiles[n - 1][j].set(false, true);
        }

        tiles[0][m - 1].set(true, true);
        tiles[n - 1][0].set(true, true);

        tiles
    }

    fn neighbors(i: usize, j: usize, n: usize, m: usize) -> Vec<(usize, usize)> {
        let mut neighs = Vec::with_capacity(4);

        if i > 0 {
            neighs.push((i - 1, j));
        }

        if i < n - 1 {
            neighs.push((i + 1, j));
        }

        if j > 0 {
            neighs.push((i, j - 1));
        }

        if j < m - 1 {
            neighs.push((i, j + 1));
        }

        neighs
    }

    fn expand_tile(i: usize, j: usize, n: usize, m: usize, tiles: &mut [Vec<Tile>]) {
        let tile = tiles[i][j].clone();

        for (ni, nj) in Self::neighbors(i, j, n, m) {
            if tile.height <= tiles[ni][nj].height && Tile::combine_oceans(&tile, &mut tiles[ni][nj]) {
                Self::expand_tile(ni, nj, n, m, tiles);
            };
        }
    }
}

fn main() {
    let input1 = vec![vec![10, 10, 10], vec![10, 1, 10], vec![10, 10, 10]];

    let input2 = vec![
        vec![1, 2, 2, 3, 5],
        vec![3, 2, 3, 4, 4],
        vec![2, 4, 5, 3, 1],
        vec![6, 7, 1, 4, 5],
        vec![5, 1, 1, 2, 4],
    ];

    let input3 = vec![
        vec![3, 3, 3, 3, 3, 3],
        vec![3, 0, 3, 3, 0, 3],
        vec![3, 3, 3, 3, 3, 3],
    ];

    let input4 = vec![
        vec![13],
        vec![4],
        vec![19],
        vec![10],
        vec![1],
        vec![11],
        vec![5],
    ];

    println!("{:?}", Solution::pacific_atlantic(input1));
    println!("{:?}", Solution::pacific_atlantic(input2));
    println!("{:?}", Solution::pacific_atlantic(input3));
    println!("{:?}", Solution::pacific_atlantic(input4));
}
