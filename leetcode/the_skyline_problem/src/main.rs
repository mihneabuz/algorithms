use std::cmp::{max, Ordering};
use std::collections::BTreeMap;
use std::collections::btree_map::Entry::Occupied;

struct Solution {}

struct Multiset {
    map: BTreeMap<i32, i32>
}

impl Multiset {
    pub fn new() -> Self {
        Self { map: BTreeMap::new() }
    }

    pub fn insert(&mut self, elem: i32) {
        self.map.entry(elem).and_modify(|x| { *x += 1 }).or_insert(1);
    }

    pub fn remove(&mut self, elem: i32) {
        if let Occupied(mut entry) = self.map.entry(elem) {
            *entry.get_mut() -= 1;
            if *entry.get() == 0 {
                entry.remove_entry();
            }
        }
    }

    pub fn max(&self) -> Option<i32> {
        self.map.iter().next_back().map(|x| *x.0)
    }
}

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut points: Vec<(i32, i32)> = buildings
            .into_iter()
            .map(|v| [(v[0], v[2]), (v[1], -v[2])])
            .flatten()
            .collect();

        points.sort();

        let mut heights = Multiset::new();
        let mut res = Vec::new();

        for point in points.into_iter() {
            if point.1 > 0 {
                heights.insert(point.1);
            } else {
                heights.remove(-point.1);
            }

            if res.len() == 0 {
                res.push(vec![point.0, point.1]);
                continue;
            }

            let (pos, height) = (point.0, heights.max().unwrap_or(0));
            let last = res.last_mut().unwrap();

            if pos == last[0] {
                last[1] = height;

                if res.len() >= 2 {
                    if res[res.len() - 2][1] == res[res.len() - 1][1] {
                        res.pop();
                    }
                }

            } else if height != last[1] {
                res.push(vec![pos, height]);
            }
        }

        res
    }

    fn get_skyline_heap(buildings: &[(i32, i32, i32)]) -> Vec<(i32, i32)> {
        let mut points: Vec<(i32, i32)> = buildings
            .into_iter()
            .map(|b| [(b.0, b.2), (b.1, -b.2)])
            .flatten()
            .collect();

        points.sort();

        let mut heights: Multiset = Multiset::new();
        let mut res: Vec<(i32, i32)> = Vec::new();

        for point in points.into_iter() {
            if point.1 > 0 {
                heights.insert(point.1);
            } else {
                heights.remove(-point.1);
            }

            if res.len() == 0 {
                res.push(point);
                continue;
            }

            let (pos, height) = (point.0, heights.max().unwrap_or(0));
            let last = res.last_mut().unwrap();

            if pos == last.0 {
                last.1 = height;

                if res.len() >= 2 {
                    if res[res.len() - 2].1 == res[res.len() - 1].1 {
                        res.pop();
                    }
                }

            } else if height != last.1 {
                res.push((pos, height));
            }
        }

        res
    }

    fn get_skyline_recursive(buildings: &[(i32, i32, i32)]) -> Vec<(i32, i32)> {
        match buildings.len() {
            0 => vec![],
            1 => {
                let building = buildings[0];
                vec![(building.0, building.2), (building.1, 0)]
            }
            n => {
                let mut res: Vec<(i32, i32)> = Vec::with_capacity(n);
                let left = Solution::get_skyline_recursive(&buildings[..n / 2]);
                let right = Solution::get_skyline_recursive(&buildings[n / 2..]);

                let mut left_iter = left.into_iter().peekable();
                let mut right_iter = right.into_iter().peekable();
                let (mut left_height, mut right_height) = (0, 0);

                while let (Some(left_point), Some(right_point)) = (left_iter.peek(), right_iter.peek()) {
                    let point = match left_point.0.cmp(&right_point.0) {
                        Ordering::Equal => {
                            left_height = max(left_height, left_point.1);
                            right_height = max(right_height, right_point.1);

                            let left_point = left_iter.next().unwrap();
                            let right_point = right_iter.next().unwrap();

                            (left_point.0, max(left_point.1, right_point.1))
                        }

                        Ordering::Less => {
                            let left_point = left_iter.next().unwrap();
                            left_height = left_point.1;
                            (left_point.0, max(left_point.1, right_height))
                        }

                        Ordering::Greater => {
                            let right_point = right_iter.next().unwrap();
                            right_height = right_point.1;
                            (right_point.0, max(right_point.1, left_height))
                        }
                    };

                    if point.1 != res.last().map(|x| (*x).1).unwrap_or(-1) {
                        res.push(point);
                    }
                }

                res.extend(left_iter);
                res.extend(right_iter);
                res
            }
        }
    }
}

fn main() {
    let inputs = [
        vec![vec![2, 9, 10], vec![3, 7, 15], vec![5, 12, 12], vec![15, 20, 10], vec![19, 24, 8]],
        vec![vec![0, 2, 3], vec![2, 5, 3]],
        vec![vec![2, 14, 4], vec![4, 8, 8], vec![6, 16, 4]],
        vec![vec![0, 2, 3], vec![2, 5, 3]],
        vec![vec![1, 2, 1], vec![1, 2, 2], vec![1, 2, 3]]
    ];

    for input in inputs {
        println!("{:?}", input.clone());
        println!("{:?}\n", Solution::get_skyline(input));
    }
}
