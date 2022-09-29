struct Solution {}

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        if arr.len() == k {
            return arr;
        }

        let (mut lo, mut hi) = (0, arr.len() - k);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;

            if x - arr[mid] > arr[mid + k] - x {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        Vec::from(&arr[lo..lo + k])
    }
}

fn main() {
    let inputs = [(vec![1, 2, 3, 4, 5], 4, 3), (vec![1, 2, 3, 4, 5], 4, -1)];

    for (arr, k, x) in inputs {
        println!("{:?}", Solution::find_closest_elements(arr, k, x));
    }
}
