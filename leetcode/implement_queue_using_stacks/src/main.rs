struct MyQueue {
    front: Vec<i32>,
    back: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        Self { front: Vec::new(), back: Vec::new() }
    }

    fn push(&mut self, x: i32) {
        self.back.push(x);
    }

    fn pop(&mut self) -> i32 {
        if self.front.is_empty() {
            self.front.extend(self.back.drain(..).rev());
        }

        self.front.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        if self.front.is_empty() {
            self.front.extend(self.back.drain(..).rev());
        }

        self.front.last().copied().unwrap()
    }

    fn empty(&self) -> bool {
        self.front.is_empty() && self.back.is_empty()
    }
}

fn main() {}
