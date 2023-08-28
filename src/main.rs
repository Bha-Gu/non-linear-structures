mod binary_tree;
use binary_tree::BinaryTree;

fn main() {
    let a = BinaryTree::from_vec_dfs(&[1, 2, 3, 4, 5, 6, 7]);
    println!("{a:?}");
}
