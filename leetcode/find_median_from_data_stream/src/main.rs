use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

struct MedianFinderSimple {
    vec: Vec<i32>,
}

impl MedianFinderSimple {
    fn new() -> Self {
        Self { vec: Vec::new() }
    }

    fn add_num(&mut self, num: i32) {
        let i = match self.vec.binary_search(&num) {
            Ok(i) => i,
            Err(i) => i,
        };

        self.vec.insert(i, num);
    }

    fn find_median(&self) -> f64 {
        let n = self.vec.len();
        if n % 2 == 0 {
            (self.vec[n / 2] as f64 + self.vec[n / 2 - 1] as f64) / 2.
        } else {
            self.vec[n / 2] as f64
        }
    }
}

struct MedianFinder {
    bigger: BinaryHeap<Reverse<i32>>,
    smaller: BinaryHeap<i32>,
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            bigger: BinaryHeap::new(),
            smaller: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        if num > self.bigger.peek().map(|x| x.0).unwrap_or(i32::MIN) {
            self.bigger.push(Reverse(num));
        } else {
            self.smaller.push(num);
        }

        if self.bigger.len() > self.smaller.len() + 1 {
            self.smaller.push(self.bigger.pop().unwrap().0);
        }

        if self.smaller.len() > self.bigger.len() + 1 {
            self.bigger.push(Reverse(self.smaller.pop().unwrap()));
        }
    }

    fn find_median(&self) -> f64 {
        match self.bigger.len().cmp(&self.smaller.len()) {
            Ordering::Equal => {
                (self.bigger.peek().unwrap().0 as f64 + *self.smaller.peek().unwrap() as f64) / 2.
            }
            Ordering::Less => *self.smaller.peek().unwrap() as f64,
            Ordering::Greater => self.bigger.peek().unwrap().0 as f64,
        }
    }
}

fn main() {
    let mut mf = MedianFinder::new();
    mf.add_num(1);
    mf.add_num(2);
    println!("{}", mf.find_median());
    mf.add_num(3);
    println!("{}", mf.find_median());
}
