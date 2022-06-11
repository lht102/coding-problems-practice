#[derive(Default)]
struct MyQueue {
    st1: Vec<i32>,
    st2: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        Default::default()
    }

    fn push(&mut self, x: i32) {
        self.st1.push(x);
    }

    fn pop(&mut self) -> i32 {
        let top = self.peek();
        self.st2.pop();
        top
    }

    fn peek(&mut self) -> i32 {
        if self.st2.is_empty() {
            while let Some(top) = self.st1.pop() {
                self.st2.push(top);
            }
        }
        *self.st2.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.st1.is_empty() && self.st2.is_empty()
    }
}
