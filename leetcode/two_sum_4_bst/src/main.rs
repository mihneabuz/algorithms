struct Solution {}

type Node = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Node,
    pub right: Node,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn find_target(root: Node, k: i32) -> bool {
        let mut seen: HashSet<i32> = HashSet::new();

        let mut queue: VecDeque<Node> = VecDeque::new();
        queue.push_back(root);

        while let Some(item) = queue.pop_front() {
            if let Some(node) = item {
                let mut node = node.borrow_mut();

                if seen.contains(&(k - node.val)) {
                    return true;
                }

                seen.insert(node.val);

                queue.push_back(node.left.take());
                queue.push_back(node.right.take());
            }
        }

        return false;
    }
}

fn main() {
    println!("Hello, world!");
}
