use stack::Stack;

pub fn run() {
    // Some text here
    let mut stack = Stack::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);

    stack.push(5);
    stack.push(4);

    println!("{}", stack);

    println!("Pop {:?}", stack.pop());

    println!("{}", stack);

}
mod stack {
    use core::fmt;

    #[derive(Debug)]
    pub struct Stack {
        top: Option<StackNode>,
        lenght: usize
    }
    impl<'a> Stack {
        pub fn new() -> Stack {
            Stack { top: None, lenght: 0 }
        }

        pub fn from_vec(numbers: Vec<i32>) -> Stack {
            let mut result = Stack::new();

            for n in numbers {
                result.push(n);
            }

            result
        }

        pub fn pop(&mut self) -> Option<i32> {
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

        pub fn push(&mut self, number: i32) {
            self.lenght += 1;
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

    impl fmt::Display for Stack {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if let Some(top) = &self.top {
                let contents = top.string_contents();

                write!(f, "{} | Length: {}", contents, self.lenght)
            } else {
                write!(f, "Empty Stack")
            }
        }
    }

    #[derive(Debug)]
    struct StackNode {
        number: i32,
        next: Option<Box<StackNode>>,
    }
    impl StackNode {
        fn string_contents(&self) -> String {
            let mut accumulator = String::new();
            if let Some(next) = &self.next {
                let next_str = next.string_contents();
                accumulator.push_str(&next_str);
                accumulator.push_str(" > ");
            }
            accumulator.push_str(&self.number.to_string());

            accumulator
        }
    }
}