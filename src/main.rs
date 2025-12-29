use tol_binary_tree::{BinaryTree, binary_search::binary_search};

fn main() {
    println!("================ Rust Binary Tree! ================\n\n");

    let mut binary_tree: BinaryTree<i32> = BinaryTree::new();

    binary_tree.add(10);
    binary_tree.add(27);
    binary_tree.add(2);
    binary_tree.add(15);
    binary_tree.add(11);
    binary_tree.add(34);
    binary_tree.add(89);
    binary_tree.add(1);
    binary_tree.add(9);
    binary_tree.add(78);
    binary_tree.add(6);
    binary_tree.add(21);
    binary_tree.add(18);
    binary_tree.add(16);
    binary_tree.add(10);
    binary_tree.add(13);
    binary_tree.add(56);
    binary_tree.add(112);
    binary_tree.add(1000);
    binary_tree.add(64);
    binary_tree.add(21);
    binary_tree.add(16);
    binary_tree.add(456);
    binary_tree.add(211);
    binary_tree.add(876);
    binary_tree.add(432);
    binary_tree.add(100);
    binary_tree.add(32);

    BinaryTree::print_tree(&binary_tree);

    println!(
        "Height of binary tree is: [{}]",
        BinaryTree::binary_tree_height(&binary_tree)
    );

    let value_to_find = 112;

    let found = binary_search(&binary_tree, &value_to_find);

    if found {
        println!("Value [{}] was found in binary tree", value_to_find)
    } else {
        println!("Value [{}] was not found in binary tree", value_to_find)
    }
}
