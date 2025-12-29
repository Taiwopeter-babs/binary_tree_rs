use std::{
    cell::RefCell,
    cmp, fmt,
    ops::{Deref, DerefMut},
    rc::{Rc, Weak},
};

/// Node type alias
pub type NodeRef<T> = Option<Rc<RefCell<TreeNode<T>>>>;

/// Parent Node type alias
type ParentPointer<T> = Option<Weak<RefCell<TreeNode<T>>>>;

#[derive(Debug)]
pub struct TreeNode<T: fmt::Display> {
    pub value: T,
    pub left: NodeRef<T>,
    pub right: NodeRef<T>,
    /// A weak reference to the parent node. This is for keeping a temporary reference to the allocation managed by Rc
    /// without preventing its inner value from being dropped. It is also used to prevent circular references between Rc pointers,
    /// since mutual owning references would never allow either Rc to be dropped.
    pub parent: ParentPointer<T>,
}

impl<T: fmt::Display> Deref for TreeNode<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: fmt::Display> DerefMut for TreeNode<T> {
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
        match &self.root {
            None => {
                let ref_cell_var = RefCell::new(TreeNode::new(value, None));

                self.root = Some(Rc::new(ref_cell_var))
            }

            Some(root_node) => {
                let weak_parent_ref = Rc::downgrade(&root_node);

                if value > root_node.borrow().value {
                    Self::insert_left(&root_node, value, Some(weak_parent_ref));
                } else {
                    Self::insert_right(&root_node, value, Some(weak_parent_ref));
                }
            }
        }
    }

    fn insert_left(
        parent: &Rc<RefCell<TreeNode<T>>>,
        value: T,
        child_parent_ref: ParentPointer<T>,
    ) {
        let mut parent_mutable_ref = parent.borrow_mut();

        match parent_mutable_ref.left {
            None => {
                let new_node = Rc::new(RefCell::new(TreeNode::new(value, child_parent_ref)));

                parent_mutable_ref.left = Some(new_node);
            }

            Some(ref left_node) => {
                let weak_left_node_ref = Rc::downgrade(&left_node);

                if value > left_node.borrow().value {
                    Self::insert_left(&left_node, value, Some(weak_left_node_ref));
                } else {
                    Self::insert_right(&left_node, value, Some(weak_left_node_ref));
                }
            }
        }
    }

    fn insert_right(
        parent: &Rc<RefCell<TreeNode<T>>>,
        value: T,
        child_parent_ref: ParentPointer<T>,
    ) {
        let mut parent_mutable_ref = parent.borrow_mut();

        match parent_mutable_ref.right {
            None => {
                let new_node = Rc::new(RefCell::new(TreeNode::new(value, child_parent_ref)));

                parent_mutable_ref.right = Some(new_node);
            }
            Some(ref right_node) => {
                if value > right_node.borrow().value {
                    Self::insert_left(&right_node, value, Some(Rc::downgrade(&right_node)));
                } else {
                    Self::insert_right(&right_node, value, Some(Rc::downgrade(&right_node)));
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
            Self::print_sideways(&n.borrow().right, depth + 1);
            println!("{:indent$}{}", "", n.borrow().value, indent = depth * 4);
            Self::print_sideways(&n.borrow().left, depth + 1);
        }
    }

    pub fn binary_tree_height(&self) -> u32 {
        let left_height = if let Some(root_node) = &self.root {
            Self::left_node_height(&root_node.borrow().left)
        } else {
            0
        };

        let right_height = if let Some(root_node) = &self.root {
            Self::right_node_height(&root_node.borrow().right)
        } else {
            0
        };

        cmp::max(right_height, left_height)
    }

    fn left_node_height(node: &NodeRef<T>) -> u32 {
        let left_node_height = if let Some(left_side) = node {
            Self::left_node_height(&left_side.borrow().left) + 1
        } else {
            0
        };

        let right_node_height = if let Some(right_side) = node {
            Self::left_node_height(&right_side.borrow().left) + 1
        } else {
            0
        };

        cmp::max(right_node_height, left_node_height)
    }

    fn right_node_height(node: &NodeRef<T>) -> u32 {
        let right_node_height = if let Some(right_side) = node {
            Self::right_node_height(&right_side.borrow().left) + 1
        } else {
            0
        };

        let left_node_height = if let Some(left_side) = node {
            Self::right_node_height(&left_side.borrow().left) + 1
        } else {
            0
        };

        cmp::max(right_node_height, left_node_height)
    }
}
