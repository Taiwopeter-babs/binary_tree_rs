use crate::{BinaryTree, binary_tree::NodeRef};
use core::fmt;

pub fn binary_search<'a, T>(tree: &BinaryTree<T>, value: &'a T) -> bool
where
    T: fmt::Display + PartialEq + PartialOrd,
{
    if tree.root.is_none() {
        return false;
    }

    let found = search_node(&tree.root, value);

    found
}

fn search_node<'a, T: fmt::Display>(node: &NodeRef<T>, value: &'a T) -> bool
where
    T: fmt::Display + PartialEq + PartialOrd,
{
    if node.is_none() {
        return false;
    }

    if let Some(node_value) = node {
        if node_value.value == *value {
            return true;
        }

        if *value > node_value.value {
            return search_node(&node_value.left, value);
        }

        if *value < node_value.value {
            return search_node(&node_value.right, value);
        }
    }

    false
}
