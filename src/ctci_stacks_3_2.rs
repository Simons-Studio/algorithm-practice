pub fn main() {
    // Some text here
    let mut stack: Stack;
}

struct Stack {
    stack: Vec<StackItem>
}

struct StackItem {
    number: i32,
    local_min: *const StackItem
}