// Question
// Given a directed graph, design an algorithm to find out weather there is a 
// root between 2 nodes 

use crate::graph::{self, Graph, Idetifier, Node};

pub fn run() {

    let string = "0 Start 1 2
1 Help 2
2 Green";

    let graph = Graph::<String>::from_string(string.to_string());
    println!("{}", graph.is_path_using_identifier(Idetifier(1), Idetifier(2)));

}