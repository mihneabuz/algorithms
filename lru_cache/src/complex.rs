use std::collections::HashMap;

#[derive(Debug)]
struct Entry<T> {
    val: T,
    next: usize,
    prev: usize,
}

impl<T> Entry<T> {
    fn new(val: T, next: usize, prev: usize) -> Self {
        Self { val, prev, next }
    }
}

#[derive(Debug)]
struct VecList<T> {
    vec: Vec<Entry<T>>,
    head: usize,
    tail: usize,
}

impl<T> VecList<T> {
    fn new(capacity: usize) -> Self {
        if capacity == 0 {
            panic!("capacity cannot be 0");
        }

        Self {
            vec: Vec::with_capacity(capacity),
            head: 0,
            tail: 0,
        }
    }

    fn head(&self) -> (usize, &T) {
        (self.head, &self.vec[self.head].val)
    }

    fn tail(&self) -> (usize, &T) {
        (self.tail, &self.vec[self.tail].val)
    }

    fn get(&self, i: usize) -> &T {
        &self.vec[i].val
    }

    fn get_mut(&mut self, i: usize) -> &mut T {
        &mut self.vec[i].val
    }

    fn push_front(&mut self, val: T) -> usize {
        if self.vec.len() == 0 {
            self.vec.push(Entry::new(val, 0, 0));
            return 0;
        }

        if self.vec.len() < self.vec.capacity() {
            let idx = self.vec.len();
            self.vec.push(Entry::new(val, self.head, self.tail));

            self.vec[self.head].prev = idx;
            self.vec[self.tail].next = idx;

            self.head = idx;
            return idx;
        }

        self.vec[self.tail].val = val;
        self.head = self.tail;
        self.tail = self.vec[self.tail].prev;

        return self.head;
    }

    fn move_to_front(&mut self, idx: usize) {
        if idx == self.head {
            return;
        }

        if idx == self.tail {
            self.head = self.tail;
            self.tail = self.vec[self.tail].prev;
            return;
        }

        let next = self.vec[idx].next;
        let prev = self.vec[idx].prev;

        if next != prev {
            self.vec[next].prev = prev;
            self.vec[prev].next = next;

            self.vec[idx].next = self.head;
            self.vec[idx].prev = self.tail;

            self.vec[self.head].prev = idx;
            self.vec[self.tail].next = idx;
        }

        self.head = idx;
    }
}

pub struct LRUCache<T> {
    map: HashMap<usize, usize>,
    vec: VecList<(usize, T)>,
}

impl<T: Copy> LRUCache<T> {
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            map: HashMap::new(),
            vec: VecList::new(capacity),
        }
    }

    pub fn get(&mut self, key: usize) -> Option<T> {
        let idx = self.map.get(&key)?;
        let entry = self.vec.get(*idx).to_owned();

        if entry.0 == key {
            self.vec.move_to_front(*idx);
            Some(entry.1)
        } else {
            None
        }
    }

    pub fn put(&mut self, key: usize, val: T) {
        match self.map.get(&key) {
            None => {
                let idx = self.vec.push_front((key, val));
                self.map.insert(key, idx);
            }
            Some(idx) => {
                let entry = self.vec.get_mut(*idx);
                if entry.0 == key {
                    entry.1 = val;
                    self.vec.move_to_front(*idx);
                } else {
                    let idx = self.vec.push_front((key, val));
                    self.map.insert(key, idx);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vecl_push_capacity() {
        let mut l = VecList::new(4);

        assert_eq!(l.push_front(1), 0);
        assert_eq!(l.push_front(2), 1);
        assert_eq!(l.push_front(3), 2);
        assert_eq!(l.push_front(4), 3);

        assert_eq!(l.head(), (3, &4));
        assert_eq!(l.tail(), (0, &1));
    }

    #[test]
    fn vecl_push_over_capacity() {
        let mut l = VecList::new(4);

        assert_eq!(l.push_front(1), 0);
        assert_eq!(l.push_front(2), 1);
        assert_eq!(l.push_front(3), 2);
        assert_eq!(l.push_front(4), 3);

        assert_eq!(l.push_front(4), 0);
        assert_eq!(l.push_front(5), 1);

        assert_eq!(l.head(), (1, &5));
        assert_eq!(l.tail(), (2, &3));
    }

    #[test]
    fn vecl_move_to_front() {
        let mut l = VecList::new(4);

        assert_eq!(l.push_front(1), 0);
        assert_eq!(l.push_front(2), 1);
        assert_eq!(l.push_front(3), 2);
        assert_eq!(l.push_front(4), 3);

        l.move_to_front(2);

        assert_eq!(l.head(), (2, &3));
        assert_eq!(l.tail(), (0, &1));

        assert_eq!(l.push_front(4), 0);
        assert_eq!(l.push_front(5), 1);

        assert_eq!(l.head(), (1, &5));
        assert_eq!(l.tail(), (3, &4));

        l.move_to_front(3);

        assert_eq!(l.head(), (3, &4));
        assert_eq!(l.tail(), (2, &3));
    }

    #[test]
    fn lru_put_and_get() {
        let mut lru = LRUCache::new(4);

        lru.put(1, 10);
        lru.put(2, 20);
        lru.put(5, 50);
        lru.put(9, 90);

        assert_eq!(lru.get(1), Some(10));
        assert_eq!(lru.get(5), Some(50));
        assert_eq!(lru.get(4), None);
        assert_eq!(lru.get(2), Some(20));
        assert_eq!(lru.get(7), None);
        assert_eq!(lru.get(9), Some(90));
    }

    #[test]
    fn lru_invalidate() {
        let mut lru = LRUCache::new(4);

        lru.put(1, 10);
        lru.put(2, 20);
        lru.put(5, 50);
        lru.put(9, 90);

        lru.put(7, 70);
        lru.put(100, 100);

        assert_eq!(lru.get(1), None);
        assert_eq!(lru.get(2), None);
        assert_eq!(lru.get(5), Some(50));
        assert_eq!(lru.get(7), Some(70));
        assert_eq!(lru.get(100), Some(100));
        assert_eq!(lru.get(9), Some(90));
    }

    #[test]
    fn lru_simple() {
        let mut cache = LRUCache::new(3);

        cache.put(1, 1);
        cache.put(2, 2);
        cache.put(3, 3);
        cache.put(4, 4);

        assert_eq!(cache.get(4), Some(4));
        assert_eq!(cache.get(3), Some(3));
        assert_eq!(cache.get(2), Some(2));
        assert_eq!(cache.get(1), None);

        cache.put(5, 5);

        assert_eq!(cache.get(1), None);
        assert_eq!(cache.get(2), Some(2));
        assert_eq!(cache.get(3), Some(3));
        assert_eq!(cache.get(4), None);
        assert_eq!(cache.get(5), Some(5));
    }

    #[test]
    fn lru_complex() {
        let mut cache = LRUCache::new(10);

        cache.put(10, 13);
        cache.put(3, 17);
        cache.put(6, 11);
        cache.put(10, 5);
        cache.put(9, 10);

        assert_eq!(cache.get(13), None);

        cache.put(2, 19);

        assert_eq!(cache.get(2), Some(19));
        assert_eq!(cache.get(3), Some(17));

        cache.put(5, 25);

        assert_eq!(cache.get(8), None);

        cache.put(9, 22);
        cache.put(5, 5);
        cache.put(1, 30);

        assert_eq!(cache.get(11), None);

        cache.put(9, 12);

        assert_eq!(cache.get(7), None);
        assert_eq!(cache.get(5), Some(5));
        assert_eq!(cache.get(8), None);
        assert_eq!(cache.get(9), Some(12));

        cache.put(4, 30);
        cache.put(9, 3);

        assert_eq!(cache.get(9), Some(3));
        assert_eq!(cache.get(10), Some(5));
        assert_eq!(cache.get(10), Some(5));

        cache.put(6, 14);
        cache.put(3, 1);

        assert_eq!(cache.get(3), Some(1));

        cache.put(10, 11);

        assert_eq!(cache.get(8), None);

        cache.put(2, 14);

        assert_eq!(cache.get(1), Some(30));
        assert_eq!(cache.get(5), Some(5));
        assert_eq!(cache.get(4), Some(30));

        cache.put(11, 4);
        cache.put(12, 24);
        cache.put(5, 18);

        assert_eq!(cache.get(13), None);

        cache.put(7, 23);

        assert_eq!(cache.get(8), None);
        assert_eq!(cache.get(12), Some(24));

        cache.put(3, 27);
        cache.put(2, 12);

        assert_eq!(cache.get(5), Some(18));

        cache.put(2, 9);
        cache.put(13, 4);
        cache.put(8, 18);
        cache.put(1, 7);

        assert_eq!(cache.get(6), None);

        cache.put(9, 29);
        cache.put(8, 21);

        assert_eq!(cache.get(5), Some(18));

        cache.put(6, 30);
        cache.put(1, 12);

        assert_eq!(cache.get(10), None);
    }
}
