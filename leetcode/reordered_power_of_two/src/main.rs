use std::cmp::Ordering;
use num::PrimInt;

struct DigitsIterator<T> {
    number: T
}

impl<T: PrimInt + From<i32>> DigitsIterator<T> {
    pub fn new(n: T) -> Self {
        Self { number: n }
    }
}

impl<T: PrimInt + From<i32>> Iterator for DigitsIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.number.is_zero() {
            None
        } else {
            let digit = self.number % 10.into();
            self.number = self.number / 10.into();
            Some(digit)
        }
    }
}

trait Digits {
    fn digits(&self) -> DigitsIterator<Self> where Self: Sized;
}

impl<T: PrimInt + From<i32>> Digits for T {
    fn digits(&self) -> DigitsIterator<Self> where Self: Sized {
        DigitsIterator::new(*self)
    }
}

struct Solution {}

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let digits = n.digits().count();

        let mut pow = 1;
        loop {
            let value = pow;
            let value_digits = value.digits().count();

            pow *= 2;

            match digits.cmp(&value_digits) {
                Ordering::Equal => {
                    if Self::can_reorder(n, value) {
                        return true
                    }
                },

                Ordering::Less    => break,
                Ordering::Greater => continue,
            }
        }

        false
    }

    fn can_reorder(n: i32, target: i32) -> bool {
        let mut appearances = vec![0; 10];

        for digit in target.digits() {
            appearances[digit as usize % 10] += 1;
        }

        for digit in n.digits() {
            if appearances[digit as usize % 10] <= 0 {
                return false;
            }

            appearances[n as usize % 10] -= 1;
        }

        true
    }
}

fn main() {
    println!("{} {}", 1, Solution::reordered_power_of2(1));
    println!("{} {}", 10, Solution::reordered_power_of2(10));
    println!("{} {}", 191820812, Solution::reordered_power_of2(10));
}
