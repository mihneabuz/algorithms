use std::collections::HashMap;

#[derive(Clone)]
struct Entry {
    key: i32,
    val: i32,
    prev: Option<usize>,
    next: Option<usize>,
}

pub struct LRUCache {
    capacity: i32,
    map: HashMap<i32, usize>,
    vec: Vec<Entry>,
    first: usize,
    last: usize,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity,
            map: HashMap::new(),
            vec: Vec::new(),
            first: 0,
            last: 0,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.map.get(&key) {
            Some(&idx) => {
                let entry = self.vec[idx].clone();

                if entry.key == key {
                    self.move_to_front(idx);
                    entry.val
                } else {
                    -1
                }
            }
            None => -1,
        }
    }

    fn put(&mut self, key: i32, val: i32) {
        if self.vec.is_empty() {
            self.vec.push(Entry {
                key,
                val,
                next: None,
                prev: None,
            });
            self.map.insert(key, 0);
        } else {
            match self.map.get(&key) {
                None => {
                    self.add_to_front(key, val);
                }
                Some(idx) => {
                    if self.vec[*idx].key == key {
                        self.vec[*idx].val = val;
                        self.move_to_front(*idx);
                    } else {
                        self.add_to_front(key, val);
                    }
                }
            }
        }
    }

    fn add_to_front(&mut self, key: i32, val: i32) {
        if self.vec.len() == self.capacity as usize {
            self.vec[self.last].key = key;
            self.vec[self.last].val = val;
            self.first = self.last;
            self.last = self.vec[self.last].prev.unwrap_or(0);
        } else {
            let idx = self.vec.len();
            self.vec.push(Entry {
                key,
                val,
                prev: Some(self.last),
                next: Some(self.first),
            });
            self.vec[self.last].next = Some(idx);
            self.vec[self.first].prev = Some(idx);
            self.first = idx;
        }

        self.map.insert(key, self.first);
    }

    fn move_to_front(&mut self, idx: usize) {
        if self.first == idx {
            return;
        }

        if self.last == idx {
            self.first = self.last;
            self.last = self.vec[idx].prev.unwrap();

            return;
        }

        let next = self.vec[idx].next;
        let prev = self.vec[idx].prev;

        if next != prev {
            if let Some(next_idx) = next {
                self.vec[next_idx].prev = prev;
            }

            if let Some(prev_idx) = prev {
                self.vec[prev_idx].next = next;
            }

            self.vec[idx].next = Some(self.first);
            self.vec[idx].prev = Some(self.last);

            self.vec[self.first].prev = Some(idx);
            self.vec[self.last].next = Some(idx);
        }

        self.first = idx;
    }
}

#[cfg(test)]
mod tests {
    use super::LRUCache;

    #[test]
    fn sample() {
        let mut cache = LRUCache::new(2);

        cache.put(1, 1);
        cache.put(2, 2);

        assert_eq!(cache.get(1), 1);

        cache.put(3, 3);

        assert_eq!(cache.get(2), -1);

        cache.put(4, 4);

        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);

    }

    #[test]
    fn overwrite() {
        let mut cache = LRUCache::new(2);

        cache.put(2, 1);
        cache.put(2, 2);

        assert_eq!(cache.get(2), 2);

        cache.put(1, 1);
        cache.put(4, 2);

        assert_eq!(cache.get(2), -1);
    }

    #[test]
    fn complex() {
        let mut cache = LRUCache::new(3);

        cache.put(1, 1);
        cache.put(2, 2);
        cache.put(3, 3);
        cache.put(4, 4);

        assert_eq!(cache.get(4), 4);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(2), 2);
        assert_eq!(cache.get(1), -1);

        cache.put(5, 5);

        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(2), 2);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), -1);
        assert_eq!(cache.get(5), 5);
    }

    #[test]
    fn complexer() {
        let mut cache = LRUCache::new(10);

        cache.put(10, 13);
        cache.put(3, 17);
        cache.put(6, 11);
        cache.put(10, 5);
        cache.put(9, 10);

        assert_eq!(cache.get(13), -1);

        cache.put(2, 19);

        assert_eq!(cache.get(2), 19);
        assert_eq!(cache.get(3), 17);

        cache.put(5, 25);

        assert_eq!(cache.get(8), -1);

        cache.put(9, 22);
        cache.put(5, 5);
        cache.put(1, 30);

        assert_eq!(cache.get(11), -1);

        cache.put(9, 12);

        assert_eq!(cache.get(7), -1);
        assert_eq!(cache.get(5), 5);
        assert_eq!(cache.get(8), -1);
        assert_eq!(cache.get(9), 12);

        cache.put(4, 30);
        cache.put(9, 3);

        assert_eq!(cache.get(9), 3);
        assert_eq!(cache.get(10), 5);
        assert_eq!(cache.get(10), 5);

        cache.put(6, 14);
        cache.put(3, 1);

        assert_eq!(cache.get(3), 1);

        cache.put(10, 11);

        assert_eq!(cache.get(8), -1);

        cache.put(2, 14);

        assert_eq!(cache.get(1), 30);
        assert_eq!(cache.get(5), 5);
        assert_eq!(cache.get(4), 30);

        cache.put(11, 4);
        cache.put(12, 24);
        cache.put(5, 18);

        assert_eq!(cache.get(13), -1);

        cache.put(7, 23);

        assert_eq!(cache.get(8), -1);
        assert_eq!(cache.get(12), 24);

        cache.put(3, 27);
        cache.put(2, 12);

        assert_eq!(cache.get(5), 18);

        cache.put(2, 9);
        cache.put(13, 4);
        cache.put(8, 18);
        cache.put(1, 7);

        assert_eq!(cache.get(6), -1);

        cache.put(9, 29);
        cache.put(8, 21);

        assert_eq!(cache.get(5), 18);

        cache.put(6, 30);
        cache.put(1, 12);

        assert_eq!(cache.get(10), -1);
    }
}
