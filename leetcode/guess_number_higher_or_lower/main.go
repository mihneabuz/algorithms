package main

func guess(num int) int {
  // given function
  return 0
}

func guessNumber(n int) int {
    lo, hi := 0, n
    for lo <= hi {
        mid := (lo + hi) / 2

        res := guess(mid)
        if res == 0 {
            return mid
        }
        if res > 0 {
            lo = mid + 1
        }
        if res < 0 {
            hi = mid
        }
    }

    return 0
}
