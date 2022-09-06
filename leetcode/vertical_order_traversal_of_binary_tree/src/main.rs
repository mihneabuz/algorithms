use std::collections::VecDeque;
use std::cmp::Ordering;

#[derive(Debug)]
struct TreeNode<T> {
    val: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    pub fn new(val: T, left: Option<Box<TreeNode<T>>>, right: Option<Box<TreeNode<T>>>) -> Self {
        Self { val, left, right }
    }

    pub fn leaf(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
struct Tree<T> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T: Copy + Ord> Tree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn vertical_traversal(&self) -> Vec<Vec<T>> {
        match &self.root {
            None => vec![],
            Some(node) => {
                let mut positive_columns: Vec<Vec<(T, i32)>> = Vec::new();
                let mut negative_columns: Vec<Vec<(T, i32)>> = Vec::new();

                let mut queue = VecDeque::new();
                queue.push_back((node, 0i32, 0i32));

                while let Some((node, row, col)) = queue.pop_front() {

                    if col < 0 {
                        let index = col.abs() - 1;
                        match negative_columns.get_mut(index as usize) {
                            Some(vec) => vec.push((node.val, row)),
                            None => negative_columns.push(vec![(node.val, row)]),
                        }
                    } else {
                        match positive_columns.get_mut(col as usize) {
                            Some(vec) => vec.push((node.val, row)),
                            None => positive_columns.push(vec![(node.val, row)]),
                        }
                    }

                    if let Some(left) = &node.left {
                        queue.push_back((left, row + 1, col - 1));
                    }

                    if let Some(right) = &node.right {
                        queue.push_back((right, row + 1, col + 1));
                    }
                }

                for vec in negative_columns.iter_mut().chain(positive_columns.iter_mut()) {
                    vec.sort_by(|x, y| {
                        match x.1.cmp(&y.1) {
                            Ordering::Less => Ordering::Less,
                            Ordering::Greater => Ordering::Greater,
                            Ordering::Equal => x.0.cmp(&y.0),
                        }
                    });
                }

                negative_columns
                    .into_iter()
                    .rev()
                    .chain(positive_columns.into_iter())
                    .map(|col| col.into_iter().map(|(val, _)| val).collect())
                    .collect()
            }
        }
    }
}

impl<T> From<TreeNode<T>> for Option<Box<TreeNode<T>>> {
    fn from(n: TreeNode<T>) -> Self {
        Some(Box::new(n))
    }
}

fn main() {
    let tree1 = Tree {
        root: TreeNode::new(
            3,
            TreeNode::new(9, None, None).into(),
            TreeNode::new(10, TreeNode::leaf(15).into(), TreeNode::leaf(7).into()).into(),
        )
        .into(),
    };

    let tree2 = Tree {
        root: TreeNode::new(
            1,
            TreeNode::new(2, TreeNode::leaf(4).into(), TreeNode::leaf(5).into()).into(),
            TreeNode::new(3, TreeNode::leaf(6).into(), TreeNode::leaf(7).into()).into(),
        )
        .into(),
    };

    let tree3 = Tree {
        root: TreeNode::new(
            1,
            TreeNode::new(2, TreeNode::leaf(4).into(), TreeNode::leaf(6).into()).into(),
            TreeNode::new(3, TreeNode::leaf(5).into(), TreeNode::leaf(7).into()).into(),
        )
        .into(),
    };

    let trees = vec![tree1, tree2, tree3];

    for tree in trees {
        println!("{:?}", tree.vertical_traversal());
    }
}
