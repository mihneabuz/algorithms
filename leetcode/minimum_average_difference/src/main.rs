struct Solution {}

struct Avg {
    sum: i64,
    count: i64,
}

impl Avg {
    fn new(sum: i64, count: i64) -> Self {
        Self { sum, count }
    }

    fn add(&mut self, num: i64) {
        self.sum += num;
        self.count += 1;
    }

    fn remove(&mut self, num: i64) {
        self.sum -= num;
        self.count -= 1;
    }

    fn get(&self) -> i64 {
        if self.count == 0 {
            0
        } else {
            self.sum / self.count
        }
    }
}

impl Solution {
    pub fn minimum_average_difference(nums: Vec<i32>) -> i32 {
        let mut avg_lo = Avg::new(nums[0] as i64, 1);
        let mut avg_hi = Avg::new(nums.iter().skip(1).map(|&x| x as i64).sum(), nums.len() as i64 - 1);

        let (mut res, mut idx) = ((avg_lo.get() - avg_hi.get()).abs(), 0);

        for (i, num) in nums.into_iter().enumerate().skip(1) {
            avg_lo.add(num as i64);
            avg_hi.remove(num as i64);

            let diff = (avg_lo.get() - avg_hi.get()).abs();
            if diff < res {
                res = diff;
                idx = i as i32;
            }
        }

        idx
    }
}

fn main() {
    println!("{}", Solution::minimum_average_difference(vec![0]));
    println!("{}", Solution::minimum_average_difference(vec![1]));
    println!("{}", Solution::minimum_average_difference(vec![4, 2, 0]));
    println!("{}", Solution::minimum_average_difference(vec![0, 0, 0, 0, 0]));
    println!("{}", Solution::minimum_average_difference(vec![2, 5, 3, 9, 5, 3]));
}
