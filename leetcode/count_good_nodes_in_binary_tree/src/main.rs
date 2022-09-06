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
                Some(node) => Some(Rc::new(RefCell::new(node))),
                None => None,
            },
            right: match right {
                Some(node) => Some(Rc::new(RefCell::new(node))),
                None => None,
            },
        }
    }
}

struct Solution {}

impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => Self::count_nodes(&node, node.borrow().val),
        }
    }

    fn count_nodes(node: &Rc<RefCell<TreeNode>>, max: i32) -> i32 {
        let val = node.borrow().val;

        let mut res = 0;

        if let Some(left) = &node.borrow().left {
            res += Self::count_nodes(left, std::cmp::max(max, val));
        }

        if let Some(right) = &node.borrow().right {
            res += Self::count_nodes(right, std::cmp::max(max, val));
        }

        if val >= max {
            res += 1
        }

        res
    }
}

fn main() {
    let root = TreeNode::new(3,
        Some(TreeNode::new(1,
            Some(TreeNode::leaf(3)),
            None)),
        Some(TreeNode::new(4,
            Some(TreeNode::leaf(1)),
            Some(TreeNode::leaf(5))))
    );

    println!("{}", Solution::good_nodes(Some(Rc::new(RefCell::new(root)))));
}
