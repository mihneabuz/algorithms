struct Solution {}

use std::cmp::Ordering;

static BASE: i32 = 3;

// needlessly complicated, but nice use of matches and simple binary search
impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n < 0 {
            return false;
        }

        match n {
            0 => return false,
            1 => return true,
            _ => (),
        };

        match n % 10 {
            2 | 4 | 5 | 6 | 8 => return false,
            _ => (),
        };

        let (mut left, mut right) = (1, 20);
        while left < right {
            let mid = (left + right) / 2;
            let value = BASE.pow(mid);

            match value.cmp(&n) {
                Ordering::Equal => return true,

                Ordering::Less => left = mid + 1,
                Ordering::Greater => right = mid,
            }
        }

        false
    }
}

fn main() {
    let tests = vec![
        (0, false),
        (1, true),
        (2, false),
        (3, true),
        (4, false),
        (5, false),
        (6, false),
        (7, false),
        (8, false),
        (9, true),
        (27, true),
        (29, false),
        (81, true),
        (99, false),
        (1162261467, true),
    ];

    for (input, expected) in tests {
        let result = Solution::is_power_of_three(input);
        println!("{} -> {} {}", input, result, expected);
    }
}
