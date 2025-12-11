//! # Simple Binary Tree
//!
//! `binary_tree` is a simple implementation of a binary tree in rust. It is purely for learning purposes.
//! It also includes a simple, albeit, useful implementation of binary search.

pub mod binary_search;
pub mod binary_tree;

pub use binary_search::binary_search;
pub use binary_tree::{BinaryTree, TreeNode};
