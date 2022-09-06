use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn leaf(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn new(val: i32, left: Option<TreeNode>, right: Option<TreeNode>) -> Self {
        TreeNode {
            val,
            left: match left {
                Some(node) => Some(node.into()),
                None => None,
            },
            right: match right {
                Some(node) => Some(node.into()),
                None => None,
            },
        }
    }
}

impl From<TreeNode> for Rc<RefCell<TreeNode>> {
    fn from(n: TreeNode) -> Self {
        Rc::new(RefCell::new(n))
    }
}

struct Solution {}

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn average_of_levels(root: Tree) -> Vec<f64> {
        if let None = root {
            return vec![];
        }

        let mut levels: Vec<(f64, f64)> = Vec::new();

        Self::average_of_levels_internal(&root, 0, &mut levels);

        levels
            .into_iter()
            .map(|(sum, count)| sum / count)
            .collect()
    }

    fn average_of_levels_internal(root: &Tree, level: i32, levels: &mut Vec<(f64, f64)>) {
        if let Some(node) = root {
            let node = node.borrow();

            match levels.get_mut(level as usize) {
                Some(l) => *l = (l.0 + node.val as f64, l.1 + 1.),
                None => levels.push((node.val as f64, 1.)),
            }

            Self::average_of_levels_internal(&node.left, level + 1, levels);
            Self::average_of_levels_internal(&node.right, level + 1, levels);
        }
    }
}

fn main() {
    let tree = TreeNode::new(
        3,
        Some(TreeNode::leaf(9)),
        Some(TreeNode::new(
            20,
            Some(TreeNode::leaf(15)),
            Some(TreeNode::leaf(7)),
        )),
    );

    println!("{:?}", Solution::average_of_levels(Some(tree.into())));
}
