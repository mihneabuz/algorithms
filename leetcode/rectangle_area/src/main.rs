use std::cmp;

struct Solution {}

impl Solution {
    pub fn compute_area(ax1: i32, ay1: i32, ax2: i32, ay2: i32, bx1: i32, by1: i32, bx2: i32, by2: i32) -> i32 {
        let width = cmp::min(ax2, bx2) - cmp::max(ax1, bx1);
        let height = cmp::min(ay2, by2) - cmp::max(ay1, by1);
        let overlap = cmp::max(width, 0) * cmp::max(height, 0);

        Self::area(ax1, ay1, ax2, ay2) + Self::area(bx1, by1, bx2, by2) - overlap
    }

    fn area(ax1: i32, ay1: i32, ax2: i32, ay2: i32) -> i32 {
        (ax2 - ax1) * (ay2 - ay1)
    }
}

fn main() {
    println!("Hello, world!");
}
