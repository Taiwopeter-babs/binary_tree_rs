use crate::{BinaryTree, binary_tree::NodeRef};
use core::fmt;

/// Searches for a `value` in a binary tree
///
/// # Example
/// ```
/// use tol_binary_tree::{BinaryTree, binary_search};
///
/// let mut binary_tree: BinaryTree<i32> = BinaryTree::new();
///
/// binary_tree.add(10);
/// binary_tree.add(27);
/// binary_tree.add(2);
/// binary_tree.add(15);
/// binary_tree.add(11);
/// binary_tree.add(34);
/// binary_tree.add(89);
///
/// let value1_to_find = 112;
///
/// let value2_to_find = 34;
///
/// let found1 = binary_search(&binary_tree, &value1_to_find);
///
/// let found2 = binary_search(&binary_tree, &value2_to_find);
///
/// assert_eq!(found1, false);
/// assert_eq!(found2, true);
///
/// ```
///
pub fn binary_search<T>(tree: &BinaryTree<T>, value: &T) -> bool
where
    T: fmt::Display + PartialEq + PartialOrd,
{
    search_node(&tree.root, value)
}

fn search_node<T: fmt::Display>(node: &NodeRef<T>, value: &T) -> bool
where
    T: fmt::Display + PartialEq + PartialOrd,
{
    if node.is_none() {
        return false;
    }

    if let Some(node_value) = node {
        if node_value.borrow().value == *value {
            return true;
        }

        if *value > node_value.borrow().value {
            return search_node(&node_value.borrow().left, value);
        }

        if *value < node_value.borrow().value {
            return search_node(&node_value.borrow().right, value);
        }
    }

    false
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_node_found_binary_search() {
        let mut binary_tree: BinaryTree<i32> = BinaryTree::new();

        binary_tree.add(10);
        binary_tree.add(27);
        binary_tree.add(2);
        binary_tree.add(15);

        let value_to_find = 15;

        let found = binary_search(&binary_tree, &value_to_find);

        assert_eq!(found, true)
    }
}
