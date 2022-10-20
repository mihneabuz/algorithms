use std::iter::repeat;

struct Solution {}

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut res = String::new();
        let mut i = 0;

        while num > 0 {
            res = Self::digit_to_roman(num % 10, i) + &res;
            (i, num) = (i + 1, num / 10);
        }

        res
    }

    pub fn digit_to_roman(d: i32, e: i32) -> String {
        let (one, five, ten) = Self::get_char(e);
        match d {
            0 => String::new(),
            x@1..=3 => repeat(one).take(x as usize).into_iter().collect(),
            4 => [one, five].into_iter().collect(),
            x@5..=8 => [five].into_iter().chain(repeat(one).take(x as usize - 5)).collect(),
            9 => [one, ten].into_iter().collect(),
            _ => panic!("not a digit {}!", d)
        }
    }

    pub fn get_char(e: i32) -> (char, char, char) {
        match e {
            0 => ('I', 'V', 'X'),
            1 => ('X', 'L', 'C'),
            2 => ('C', 'D', 'M'),
            3 => ('M', '!', '!'),
            _ => panic!("number too big!"),
        }
    }
}

fn main() {
    let inputs = [3, 58, 1994];

    for input in inputs {
        println!("{}", Solution::int_to_roman(input));
    }
}
