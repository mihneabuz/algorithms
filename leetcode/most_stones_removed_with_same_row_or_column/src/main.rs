use std::collections::{HashMap, HashSet};

struct Solution {}

struct Union {
    map: HashMap<i32, i32>,
}

impl Union {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn parent(&mut self, x: i32) -> &mut i32 {
        self.map.entry(x).or_insert(0)
    }

    pub fn find(&mut self, x: i32) -> i32 {
        match *self.parent(x) {
            0 => x,
            p => self.find(p)
        }
    }

    pub fn union(&mut self, x: i32, y: i32) {
        let (x_un, y_un) = (self.find(x), self.find(y));
        if x_un != y_un {
            *self.parent(x_un) = y_un;
        }
    }
}

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let mut dsu = Union::new();
        for stone in stones.iter() {
            dsu.union(stone[0], -stone[1] - 1);
        }

        let total = stones.len();
        let distinct = stones.into_iter().map(|s| dsu.find(s[0])).collect::<HashSet<i32>>().len();
        (total - distinct) as i32
    }
}

fn main() {
    let inputs = [vec![
        vec![0, 0],
        vec![0, 1],
        vec![1, 0],
        vec![1, 2],
        vec![2, 1],
        vec![2, 2],
    ]];

    for input in inputs {
        println!("{}", Solution::remove_stones(input));
    }
}
