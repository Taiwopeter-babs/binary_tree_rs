use std::{
    cmp, fmt,
    ops::{Deref, DerefMut},
};

/// Node type alias
pub type NodeRef<T> = Option<Box<TreeNode<T>>>;

/// Parent Node type alias
type ParentPointer<T> = *mut TreeNode<T>;

#[derive(Debug)]
pub struct TreeNode<T: fmt::Display> {
    pub value: T,
    pub left: NodeRef<T>,
    pub right: NodeRef<T>,
    pub parent: ParentPointer<T>,
}

impl<T: fmt::Display> Deref for TreeNode<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: fmt::Display> DerefMut for TreeNode<T> {
    // fn deref_mut(&mut self) -> &mut Self::Target {
    //     &mut self.value
    // }

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: fmt::Display> fmt::Display for TreeNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.value)
    }
}

impl<T: fmt::Display> TreeNode<T> {
    /// Creates a new binary tree node.
    pub fn new(value: T, parent: ParentPointer<T>) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
            parent,
        }
    }
}

pub struct BinaryTree<T: fmt::Display> {
    pub root: NodeRef<T>,
}

impl<T: fmt::Display + PartialOrd> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree { root: None }
    }

    pub fn add(&mut self, value: T) {
        match self.root {
            None => self.root = Some(Box::new(TreeNode::new(value, std::ptr::null_mut()))),

            Some(ref mut root_node) => {
                if value > root_node.value {
                    Self::insert_left(root_node, value);
                } else {
                    Self::insert_right(root_node, value);
                }
            }
        }
    }

    fn insert_left(parent: &mut Box<TreeNode<T>>, value: T) {
        match parent.left {
            None => {
                let parent_ptr = &mut **parent;

                let new_node = Some(Box::new(TreeNode::new(value, parent_ptr)));

                parent.left = new_node;
            }
            Some(ref mut left_node) => {
                if value > left_node.value {
                    Self::insert_left(left_node, value);
                } else {
                    Self::insert_right(left_node, value);
                }
            }
        }
    }

    fn insert_right(parent: &mut Box<TreeNode<T>>, value: T) {
        match parent.right {
            None => {
                let parent_ptr = &mut **parent;

                let new_node = Some(Box::new(TreeNode::new(value, parent_ptr)));

                parent.right = new_node;
            }
            Some(ref mut right_node) => {
                if value > right_node.value {
                    Self::insert_left(right_node, value);
                } else {
                    Self::insert_right(right_node, value);
                }
            }
        }
    }

    pub fn print_tree(&self) {
        if self.root.is_none() {
            return ();
        };
        Self::print_sideways(&self.root, 0);
    }

    fn print_sideways(node: &NodeRef<T>, depth: usize) {
        if let Some(n) = node {
            Self::print_sideways(&n.right, depth + 1);
            println!("{:indent$}{}", "", n.value, indent = depth * 4);
            Self::print_sideways(&n.left, depth + 1);
        }
    }

    pub fn binary_tree_height(&self) -> u32 {
        let left_height = if let Some(root_node) = &self.root {
            Self::left_node_height(&root_node.left)
        } else {
            0
        };

        let right_height = if let Some(root_node) = &self.root {
            Self::right_node_height(&root_node.right)
        } else {
            0
        };

        cmp::max(right_height, left_height)
    }

    fn left_node_height(node: &NodeRef<T>) -> u32 {
        let left_node_height = if let Some(left_side) = node {
            Self::left_node_height(&left_side.left) + 1
        } else {
            0
        };

        let right_node_height = if let Some(right_side) = node {
            Self::left_node_height(&right_side.left) + 1
        } else {
            0
        };

        cmp::max(right_node_height, left_node_height)
    }

    fn right_node_height(node: &NodeRef<T>) -> u32 {
        let right_node_height = if let Some(right_side) = node {
            Self::right_node_height(&right_side.left) + 1
        } else {
            0
        };

        let left_node_height = if let Some(left_side) = node {
            Self::right_node_height(&left_side.left) + 1
        } else {
            0
        };

        cmp::max(right_node_height, left_node_height)
    }
}
