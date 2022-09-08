use std::cmp::Ordering;
use std::fmt::Display;

#[derive(Debug)]
struct TreeNode<T: Eq + Ord> {
    val: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Eq + Ord,
{
    pub fn leaf(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

impl<T> Display for TreeNode<T>
where
    T: Eq + Ord + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_leaf() {
            write!(f, "{}", self.val)
        } else {
            write!(f, "({} ", self.val)?;

            match &self.left {
                Some(n) => {
                    write!(f, "({}) ", n)?;
                }
                None => {
                    write!(f, "() ")?;
                }
            }

            match &self.right {
                Some(n) => {
                    write!(f, "({}))", n)
                }
                None => {
                    write!(f, "())")
                }
            }
        }
    }
}

#[derive(Debug)]
struct Tree<T: Eq + Ord> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T> Tree<T>
where
    T: Eq + Ord + Copy,
{
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn from_iter(iter: impl Iterator<Item = T>) -> Self {
        let mut tree = Tree::new();
        for val in iter {
            tree.insert(val);
        }

        tree
    }

    pub fn insert(&mut self, val: T) {
        match &mut self.root {
            None => self.root = TreeNode::leaf(val).into(),
            Some(node) => Tree::insert_recursive(node, val),
        }
    }

    fn insert_recursive(node: &mut Box<TreeNode<T>>, val: T) {
        match val.cmp(&node.val) {
            Ordering::Less => match &mut node.left {
                None => {
                    node.left = TreeNode::leaf(val).into();
                }
                Some(left) => {
                    Tree::insert_recursive(left, val);
                }
            },
            Ordering::Greater => match &mut node.right {
                None => {
                    node.right = TreeNode::leaf(val).into();
                }
                Some(right) => {
                    Tree::insert_recursive(right, val);
                }
            },

            Ordering::Equal => (),
        };
    }

    pub fn find(&self, val: T) -> bool {
        match &self.root {
            None => false,
            Some(node) => Tree::find_recursive(node, val),
        }
    }

    fn find_recursive(node: &Box<TreeNode<T>>, val: T) -> bool {
        match val.cmp(&node.val) {
            Ordering::Equal => true,

            Ordering::Less => match &node.left {
                None => false,
                Some(left) => Tree::find_recursive(left, val),
            },

            Ordering::Greater => match &node.right {
                None => false,
                Some(right) => Tree::find_recursive(right, val),
            },
        }
    }

    pub fn inorder(&self) -> Vec<T> {
        match &self.root {
            None => vec![],
            Some(node) => Tree::inorder_recursive(node),
        }
    }

    fn inorder_recursive(node: &Box<TreeNode<T>>) -> Vec<T> {
        let mut res = Vec::new();

        if let Some(left) = &node.left {
            res.extend(Tree::inorder_recursive(left));
        }

        res.push(node.val);

        if let Some(right) = &node.right{
            res.extend(Tree::inorder_recursive(right));
        }

        res
    }

    pub fn inorder_iterative(&self) -> Vec<T> {
        if self.root.is_none() {
            return Vec::new();
        }

        let mut result = Vec::new();
        let mut stack = Vec::new();
        let mut current = self.root.as_ref();

        while !stack.is_empty() || current.is_some() {
            while let Some(node) = current {
                stack.push(node);
                current = node.left.as_ref();
            }

            if let Some(node) = stack.pop() {
                result.push(node.val);
                current = node.right.as_ref();
            }
        }

        result
    }
}

impl<T> From<TreeNode<T>> for Option<Box<TreeNode<T>>>
where
    T: Eq + Ord,
{
    fn from(n: TreeNode<T>) -> Self {
        Some(Box::new(n))
    }
}

impl<T> Display for Tree<T>
where
    T: Eq + Ord + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.root {
            Some(n) => write!(f, "{}", n),
            None => write!(f, "()"),
        }
    }
}

fn main() {
    let mut tree = Tree::from_iter([3, 9, 2].into_iter());

    tree.insert(5);

    println!("{}", tree);

    println!("find {}: {}", 9, tree.find(9));
    println!("find {}: {}", 4, tree.find(4));

    println!("inorder: {:?}", tree.inorder());
    println!("inorder: {:?}", tree.inorder_iterative());
}
