struct Solution {}

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut occurences = vec![0; 2001];
        for num in arr.into_iter() {
            occurences[(num + 1000) as usize] += 1;
        }

        let mut seen = vec![false; 2001];
        for occ in occurences.into_iter().filter(|&occ| occ > 0) {
            if seen[occ] {
                return false
            }

            seen[occ] = true
        }

        true
    }
}

fn main() {
    println!("{}", Solution::unique_occurrences(vec![1,2,2,1,1,3]));
    println!("{}", Solution::unique_occurrences(vec![1,2]));
    println!("{}", Solution::unique_occurrences(vec![-3,0,1,-3,1,1,1,-3,10,0]));
}
