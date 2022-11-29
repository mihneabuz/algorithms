use std::collections::HashMap;

struct RandomizedSet {
    map: HashMap<i32, usize>,
    vec: Vec<i32>,
}

impl RandomizedSet {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            vec: Vec::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.map.get(&val).is_some() {
            return false;
        }

        self.map.insert(val, self.vec.len());
        self.vec.push(val);
        return true;
    }

    fn remove(&mut self, val: i32) -> bool {
        match self.map.get(&val) {
            None => false,
            Some(&idx) => {
                self.map.remove(&val);

                if let Some(last) = self.vec.pop() {
                    if last == val {
                        return true
                    }

                    self.vec[idx] = last;
                    self.map.entry(last).and_modify(|v| *v = idx);
                }

                true
            }
        }
    }

    fn get_random(&self) -> i32 {
        self.vec[rand::random::<usize>() % self.vec.len()]
    }
}

fn main() {
    let mut rset = RandomizedSet::new();

    rset.insert(1);
    println!("-> {:?} {:?}", rset.map, rset.vec);
    rset.insert(2);
    println!("-> {:?} {:?}", rset.map, rset.vec);
    rset.insert(2);
    println!("-> {:?} {:?}", rset.map, rset.vec);
    rset.insert(3);
    println!("-> {:?} {:?}", rset.map, rset.vec);

    println!("> {}", rset.get_random());

    rset.remove(2);
    println!("-> {:?} {:?}", rset.map, rset.vec);
    rset.remove(2);
    println!("-> {:?} {:?}", rset.map, rset.vec);
    rset.remove(1);
    println!("-> {:?} {:?}", rset.map, rset.vec);
    rset.remove(3);
    println!("-> {:?} {:?}", rset.map, rset.vec);
}
