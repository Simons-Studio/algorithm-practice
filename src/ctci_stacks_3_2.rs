use min_stack::MinStack;

pub fn run() {
    // Some text here
    let mut stack = MinStack::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);

    stack.push(5);
    stack.push(4);

    println!("{}", stack);

    println!("Pop {:?}", stack.pop());
    println!("Pop {:?}", stack.pop());
    println!("Pop {:?}", stack.pop());
    println!("Pop {:?}", stack.pop());
    println!("Pop {:?}", stack.pop());

    println!("{}", stack);

}

mod min_stack {
    use core::fmt;

    #[derive(Debug)]
    pub struct MinStack {
        top: Option<MinStackNode>,
        lenght: usize
    }
    impl MinStack {
        pub fn new() -> MinStack {
            MinStack { top: None, lenght: 0 }
        }

        pub fn from_vec(numbers: Vec<i32>) -> MinStack {
            let mut result = MinStack::new();

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
                let min = number.min(top.min);
                let node = MinStackNode { number, next: Some(Box::new(top)), min };
                self.top = Some(node);
            } else {
                let node = MinStackNode { number, next: None, min: number }; 
                self.top = Some(node);
                return;
            };
        }

        pub fn min(&self) -> Option<i32> {
            if let Some(top) = &self.top {
                return Some(top.min);
            } 
            None
        }
    }

    impl fmt::Display for MinStack {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if let Some(top) = &self.top {
                let contents = top.string_contents();

                write!(f, "{} | Length: {} | Min: {}", contents, self.lenght, top.min)
            } else {
                write!(f, "Empty Stack")
            }
        }
    }

    #[derive(Debug)]
    struct MinStackNode {
        number: i32,
        next: Option<Box<MinStackNode>>,
        min: i32,
    }
    impl MinStackNode {
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