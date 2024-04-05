// Given a sorted array (ascending order) with unique integer elements, write an algorithm to 
// create a binary search tree with minimal height

use crate::utils::trees::BinarySearchTree;

pub fn run() {
    let x = 340..1234;

    let tree = BinarySearchTree::from_array(x.collect()).unwrap();

    println!("{}", tree.to_string());
} 