#[derive(Debug)]
struct TreeNode<T> {
    val: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    pub fn leaf(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    pub fn node(val: T, left: Option<Box<TreeNode<T>>>, right: Option<Box<TreeNode<T>>>) -> Self {
        Self { val, left, right }
    }
}

#[derive(Debug)]
struct Tree<T> {
    root: Option<Box<TreeNode<T>>>,
}

impl Tree<i32> {
    pub fn prune(&mut self) {
        let sum = Self::recursive_prune(&mut self.root);

        if sum == 0 {
            self.root.take();
        }
    }

    fn recursive_prune(root: &mut Option<Box<TreeNode<i32>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let left = Self::recursive_prune(&mut node.left);
                if left == 0 {
                    node.left.take();
                }

                let right = Self::recursive_prune(&mut node.right);
                if right == 0 {
                    node.right.take();
                }

                node.val + left + right
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
    let mut tree = Tree {
        root: TreeNode::node(
            1,
            TreeNode::node(0, TreeNode::leaf(0).into(), TreeNode::leaf(0).into()).into(),
            TreeNode::node(1, TreeNode::leaf(0).into(), TreeNode::leaf(1).into()).into(),
        )
        .into(),
    };

    println!("{:?}", tree);
    tree.prune();
    println!("{:?}", tree);
}
