use std::collections::VecDeque;

struct MyCircularQueue {
    queue: VecDeque<i32>,
    size: usize,
}

impl MyCircularQueue {

    fn new(k: i32) -> Self {
        Self { queue: VecDeque::with_capacity(k as usize), size: k as usize }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.queue.len() == self.size {
            false
        } else {
            self.queue.push_back(value);
            true
        }
    }

    fn de_queue(&mut self) -> bool {
        match self.queue.pop_front() {
            Some(_) => true,
            None    => false
        }
    }

    fn front(&self) -> i32 {
        self.queue.front().map(|x| *x).unwrap_or(-1)
    }

    fn rear(&self) -> i32 {
        self.queue.back().map(|x| *x).unwrap_or(-1)
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn is_full(&self) -> bool {
        self.queue.len() == self.size
    }
}

fn main() {
    let mut queue = MyCircularQueue::new(5);

    queue.en_queue(1);
    queue.en_queue(2);
    queue.en_queue(3);

    assert_eq!(queue.is_full(), false);
    assert_eq!(queue.is_empty(), false);
    assert_eq!(queue.rear(), 3);

    assert_eq!(queue.front(), 1);
    queue.de_queue();

    assert_eq!(queue.front(), 2);
    queue.de_queue();

    assert_eq!(queue.front(), 3);
    queue.de_queue();

    assert_eq!(queue.is_empty(), true);
}

