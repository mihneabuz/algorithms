struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn leaf(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    pub fn new(val: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Self {
        Self { val, left, right }
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

struct Tree {
    root: Option<Box<TreeNode>>,
}

impl Tree {
    fn new(root: Option<Box<TreeNode>>) -> Self {
        Self { root }
    }

    fn pseudo_palindrome_paths(&self) -> i32 {
        Tree::pseudo_palindrome_recursive(&self.root, &mut vec![0; 10])
    }

    fn pseudo_palindrome_recursive(root: &Option<Box<TreeNode>>, path: &mut [i32]) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                path[node.val as usize] += 1;

                let res = if node.is_leaf() {
                    if is_pseudo_palindrome(path) { 1 } else { 0 }
                } else {
                    Tree::pseudo_palindrome_recursive(&node.left, path) + Tree::pseudo_palindrome_recursive(&node.right, path)
                };

                path[node.val as usize] -= 1;
                res
            },
        }
    }
}

fn is_pseudo_palindrome(path: &[i32]) -> bool {
    let mut found_odd = false;
    for val in path {
        if val % 2 == 1 {
            if found_odd {
                return false
            } else {
                found_odd = true;
            }
        }
    }

    return true
}

fn main() {
    let tree = Tree::new(Some(Box::new(TreeNode::new(
        2,
        Some(Box::new(TreeNode::new(
            3,
            Some(Box::new(TreeNode::leaf(3))),
            Some(Box::new(TreeNode::leaf(1))),
        ))),
        Some(Box::new(TreeNode::new(
            1,
            None,
            Some(Box::new(TreeNode::leaf(1))),
        ))),
    ))));

    println!("{}", tree.pseudo_palindrome_paths());
}

