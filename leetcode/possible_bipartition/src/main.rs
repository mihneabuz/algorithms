const NOT_COLORED: u8 = 3;
const RED: u8 = 0;

struct Solution {}

impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for dislike in dislikes {
            graph[dislike[0] as usize - 1].push(dislike[1] as usize - 1);
            graph[dislike[1] as usize - 1].push(dislike[0] as usize - 1);
        }

        let mut color = vec![NOT_COLORED; n];
        for i in 0..n {
            if color[i] == NOT_COLORED && !Self::dfs(i, RED, &graph, &mut color) {
                return false;
            }
        }

        true
    }

    fn dfs(node: usize, color: u8, graph: &[Vec<usize>], colors: &mut [u8]) -> bool {
        if colors[node] != NOT_COLORED {
            colors[node] == color
        } else {
            colors[node] = color;
            graph[node]
                .iter()
                .all(|&neigh| Self::dfs(neigh, 1 - color, graph, colors))
        }
    }
}

fn main() {
    println!("{}", Solution::possible_bipartition(4, vec![vec![1, 2], vec![1, 3], vec![2, 4]]));
    println!("{}", Solution::possible_bipartition(3, vec![vec![1, 2], vec![1, 3], vec![2, 3]]));
}
