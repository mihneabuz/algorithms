use std::collections::HashMap;

pub struct RotVec<T> {
    vec: Vec<T>,
    cursor: usize,
    capacity: usize,
}

impl<T> RotVec<T> {
    pub fn new(n: usize) -> Self {
        RotVec { vec: Vec::with_capacity(n), cursor: 0, capacity: n }
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        self.vec.get(i)
    }

    pub fn push(&mut self, e: T) -> usize {
        if self.vec.len() <= self.cursor {
            self.vec.push(e);
        } else {
            self.vec[self.cursor] = e;
        }

        let inserted_at = self.cursor;

        self.cursor += 1;
        if self.cursor == self.capacity {
            self.cursor = 0;
        }

        inserted_at
    }
}

pub struct LRUCache<T> {
    map: HashMap<usize, usize>,
    rvec: RotVec<(usize, T)>,
}

impl<T> LRUCache<T> {
    pub fn new(capacity: usize) -> Self {
        LRUCache { map: HashMap::new(), rvec: RotVec::new(capacity) }
    }

    pub fn put(&mut self, key: usize, val: T) {
        let idx = self.rvec.push((key, val));
        self.map.insert(key, idx);
    }

    pub fn get(&self, key: usize) -> Option<&T> {
        let idx = self.map.get(&key)?;
        let (fkey, fval) = self.rvec.get(*idx)?;

        if *fkey == key {
            Some(fval)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotating_vec() {
        let mut rvec = RotVec::new(4);

        rvec.push(1);
        rvec.push(2);
        rvec.push(3);

        assert_eq!(rvec.get(0), Some(&1));
        assert_eq!(rvec.get(1), Some(&2));
        assert_eq!(rvec.get(2), Some(&3));

        rvec.push(4);
        rvec.push(5);
        rvec.push(6);

        assert_eq!(rvec.get(0), Some(&5));
        assert_eq!(rvec.get(1), Some(&6));
        assert_eq!(rvec.get(2), Some(&3));
        assert_eq!(rvec.get(3), Some(&4));
    }

    #[test]
    fn lru_put_and_get() {
        let mut lru = LRUCache::new(4);

        lru.put(1, 10);
        lru.put(2, 20);
        lru.put(5, 50);
        lru.put(9, 90);

        assert_eq!(lru.get(1), Some(&10));
        assert_eq!(lru.get(5), Some(&50));
        assert_eq!(lru.get(4), None);
        assert_eq!(lru.get(2), Some(&20));
        assert_eq!(lru.get(7), None);
        assert_eq!(lru.get(9), Some(&90));
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
        assert_eq!(lru.get(5), Some(&50));
        assert_eq!(lru.get(7), Some(&70));
        assert_eq!(lru.get(100), Some(&100));
        assert_eq!(lru.get(2), None);
        assert_eq!(lru.get(9), Some(&90));
    }
}
