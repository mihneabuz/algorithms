struct Solution {}

impl Solution {
    pub fn outer_trees(mut trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        trees.sort();
        let mut l2r = Vec::with_capacity(trees.len());
        for tree in trees.iter() {
            while Self::can_prune(&l2r, tree) {
                l2r.pop();
            }
            l2r.push(tree.clone());
        }

        trees.reverse();
        let mut r2l = Vec::with_capacity(trees.len());
        for tree in trees.iter() {
            while Self::can_prune(&r2l, tree) {
                r2l.pop();
            }
            r2l.push(tree.clone());
        }

        l2r.extend_from_slice(&r2l);
        l2r.sort();
        l2r.dedup();

        l2r
    }

    fn can_prune(res: &[Vec<i32>], next: &Vec<i32>) -> bool {
        res.len() >= 2 && Self::orient(&res[res.len() - 2], &res[res.len() - 1], next)
    }

    fn orient(p: &Vec<i32>, q: &Vec<i32>, r: &Vec<i32>) -> bool {
        (q[0] - p[0]) * (r[1] - q[1]) - (q[1] - p[1]) * (r[0] - q[0]) > 0
    }
}

fn main() {
    let trees = vec![
        vec![1, 1],
        vec![2, 2],
        vec![2, 0],
        vec![2, 4],
        vec![3, 3],
        vec![4, 2],
    ];
    println!("{:?}", Solution::outer_trees(trees));
}
