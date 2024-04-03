struct Graph<T> {
    nodes: Vec<Node<T>>
}

struct Node<T> {
    idetifier: Idetifier,
    value: T,
    adjacent: Vec<Idetifier>
}

struct Idetifier(String);