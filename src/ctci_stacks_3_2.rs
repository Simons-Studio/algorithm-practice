pub fn main() {
    // Some text here
    let mut stack: Stack;
}

struct Stack<'a> {
    stack: Vec<StackItem<'a>>
}
impl<'a> Stack<'a> {
    fn pop(&mut self) -> Option<StackItem<'a>> {
        if self.stack.is_empty() { None }
        else {
            let last = self.stack.last();

            None
        }
    }

    fn push(&mut self, number: i32) {

    }
}

struct StackItem<'a> {
    number: i32,
    local_min: Option<&'a StackItem<'a>>
}