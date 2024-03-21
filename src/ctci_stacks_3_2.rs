pub fn main() {
    // Some text here
    let mut stack: Stack;
}

#[derive(Debug)]
struct Stack {
    top: Option<StackNode>,
    lenght: usize
}
impl<'a> Stack {
    fn pop(&mut self) -> Option<i32> {
        let Some(top) = self.top.take() else {
            return None;
        };

        let value = top.number;

        if let Some(next_box) = top.next {            
            self.top = Some(*next_box);
        } else {
            self.top = None;
        }

        self.lenght -= 1;

        Some(value)
    }

    fn push(&mut self, number: i32) {
        if let Some(top) = self.top.take() {
            let node = StackNode { number, next: Some(Box::new(top)) };
            self.top = Some(node);
        } else {
            let node = StackNode { number, next: None }; 
            self.top = Some(node);
            return;
        };
    }
}

#[derive(Debug)]
struct StackNode {
    number: i32,
    next: Option<Box<StackNode>>,
}