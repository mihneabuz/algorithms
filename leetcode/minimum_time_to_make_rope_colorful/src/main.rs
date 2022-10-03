struct Solution {}

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut res = 0;

        let mut last_c = 0;
        let mut times = Vec::new();

        for (c, t) in colors.bytes().zip(needed_time.into_iter()) {
            if c != last_c {
                res += Solution::remove_cost(&times);
                times.clear();
            }

            times.push(t);
            last_c = c;
        }

        res + Solution::remove_cost(&times)
    }

    fn remove_cost(needed_time: &[i32]) -> i32 {
        if needed_time.len() < 2 {
            return 0;
        }

        let max: i32 = *needed_time.iter().max().unwrap();
        let sum: i32 = needed_time.into_iter().sum();

        sum - max
    }
}

fn main() {
    let inputs = [
        (String::from("abaac"), vec![1, 2, 3, 4, 5]),
        (String::from("aba"), vec![1, 2, 3]),
        (String::from("aabaa"), vec![1, 2, 3, 4, 1]),
    ];

    for (colors, time) in inputs {
        println!("{}", Solution::min_cost(colors, time));
    }
}
