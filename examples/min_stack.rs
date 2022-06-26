use partial_functional::prelude::*;

#[derive(Debug)]
struct MinStack {
    stack: Vec<Min<i32>>,
    min: Min<i32>,
}

impl MinStack {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            min: Min::empty(),
        }
    }

    fn push(&mut self, val: i32) {
        self.min = self.min.combine(val.into());
        self.stack.push(val.into());
    }

    fn pop(&mut self) -> i32 {
        let top = self.stack.pop().unwrap();

        if top == self.min {
            self.min = self.stack.iter().fold(Min::empty(), |min, &x| min.combine(x));
        }

        top.0
    }

    fn top(&self) -> i32 {
        self.stack[self.stack.len() - 1].0
    }

    fn get_min(&self) -> i32 {
        self.min.0
    }
}

fn main() {
    let mut stack = MinStack::new();

    stack.push(-2);
    stack.push(0);
    stack.push(-3);

    println!("Min: {}", stack.get_min());
    println!("Pop: {}", stack.pop());
    println!("Top: {}", stack.top());
    println!("Min: {}", stack.get_min());
}
