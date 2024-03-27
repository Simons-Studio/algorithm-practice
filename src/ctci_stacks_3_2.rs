use crate::stack::Stack;

pub fn run() {
    // Some text here
    let mut stack = Stack::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);

    stack.push(5);
    stack.push(4);

    println!("{}", stack);

    println!("Pop {:?}", stack.pop());

    println!("{}", stack);

}   