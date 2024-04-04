// Given a sorted array (ascending order) with unique integer elements, write an algorithm to 
// create a binary search tree with minimal height

use crate::utils::trees::BinarySearchTreeNode;

pub fn run() {
    let x = 10..23;
    let y = [1, 2, 3, 4, 5, 6, 7, 8];

    let mut tree = BinarySearchTreeNode::leaf(0);
    for i in x {
        tree.insert_max(i);
    }
    println!("{}", tree.to_string());
} 