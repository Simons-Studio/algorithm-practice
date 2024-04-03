use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct Graph<T> {
    nodes: HashMap<Idetifier, Node<T>>,
}
impl<T> Graph<T> {
    pub fn from_string(string: String) -> Graph<String> {
        let mut nodes = HashMap::new();
        for line in string.lines() {
            if let Some(node) = Node::<String>::from_string(line) {
                nodes.insert(node.identifier, node);
            }
        }
        Graph{ nodes }
    }

    pub fn is_path(&self, start: &Node<T>, end: &Node<T>) -> bool {
        return self.is_path_using_identifier(start.identifier, end.identifier);
    }

    pub fn is_path_using_identifier(&self, start: Idetifier, end: Idetifier) -> bool {
        let mut visited: HashSet<Idetifier> = HashSet::new();
        let mut start_queue: VecDeque<&Node<T>> = VecDeque::new();
        let mut end_queue: VecDeque<&Node<T>> = VecDeque::new();

        if let Some(node) = self.nodes.get(&start) {
            visited.insert(start);
            start_queue.push_back(node);
        } else {
            return false;
        }

        if let Some(node) = self.nodes.get(&end) {
            visited.insert(end);
            end_queue.push_back(node);
        } else {
            return false;
        }

        while !(start_queue.is_empty() || end_queue.is_empty()) {
            let start_ancestor = start_queue.pop_front().unwrap();
            let end_ancestor = end_queue.pop_front().unwrap();

            if visited.contains(&start_ancestor.identifier)
                || visited.contains(&end_ancestor.identifier)
            {
                return true;
            } else {
                visited.insert(start_ancestor.identifier);
                visited.insert(end_ancestor.identifier);

                for adjacent_id in &start_ancestor.adjacent {
                    if let Some(adjacent_node) = self.nodes.get(adjacent_id) {
                        start_queue.push_back(adjacent_node);
                    }
                }

                for adjacent_id in &end_ancestor.adjacent {
                    if let Some(adjacent_node) = self.nodes.get(adjacent_id) {
                        end_queue.push_back(adjacent_node);
                    }
                }
            }
        }
        false
    }

    pub fn insert(&mut self, node: Node<T>) -> bool {
        if self.nodes.contains_key(&node.identifier) {
            return false;
        }

        for adjacent_id in &node.adjacent {
            if let Some(adjacent_node) = self.nodes.get_mut(&adjacent_id) {
                adjacent_node.adjacent.push(node.identifier);
            }
        }

        self.nodes.insert(node.identifier, node);

        true
    }

    fn update(&mut self, node: Node<T>) -> Option<Node<T>> {
        if !self.nodes.contains_key(&node.identifier) {
            return None;
        } 

        // TODO: remove old links

        for adjacent_id in &node.adjacent {
            if let Some(adjacent_node) = self.nodes.get_mut(&adjacent_id) {
                adjacent_node.adjacent.push(node.identifier);
            }
        }

        self.nodes.insert(node.identifier, node)
    }
}

#[derive(Debug)]
pub struct Node<T> {
    identifier: Idetifier,
    value: T,
    adjacent: Vec<Idetifier>,
}
impl<T> Node<T> {
    fn from_string(string: &str) -> Option<Node<String>> {
        let mut parts = string.split_ascii_whitespace();
        let id_str = parts.next().unwrap_or("");
        let value = parts.next().unwrap_or("").to_string();

        if id_str.is_empty() || value.is_empty() {
            return None;
        }

        let identifier = Idetifier(id_str.parse().expect("id_str should be of type usize"));
        let mut adjacent = Vec::new();

        for adjacent_str in parts {
            adjacent.push(Idetifier(adjacent_str.parse().expect("adjacent identifiers must be of type usize")));
        }

        Some(Node{ identifier, value, adjacent })
    }

    fn from_tuple(tuple: (Idetifier, T, Vec<Idetifier>)) -> Node<T> {
        let (identifier, value, adjacent) = tuple;
        Node { identifier, value, adjacent }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
pub struct Idetifier(pub usize);
