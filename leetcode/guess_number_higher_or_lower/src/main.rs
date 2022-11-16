
/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

struct Solution {}

fn guess(num: i32) -> i32 {
    // given function
    return 0;
}

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let (mut lo, mut hi) = (1, n);
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;

            match guess(mid) {
                 0 => return mid,
                 1 => { lo = mid + 1 },
                -1 => { hi = mid },
                 _ => return -1
            }
        }

        return -1;
    }
}

fn main() {
}
