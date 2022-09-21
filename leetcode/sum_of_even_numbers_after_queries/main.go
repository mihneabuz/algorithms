package main

func sumEvenAfterQueries(nums []int, queries [][]int) []int {
  res := make([]int, 0, len(nums))

  sum_even := 0
  for _, num := range nums {
    if num % 2 == 0 {
      sum_even += num;
    }
  }

  for _, q := range queries {
    val, idx := q[0], q[1]
    num := nums[idx]

    if num % 2 == 0 {
      sum_even -= num
    }

    num += val

    if num % 2 == 0 {
      sum_even += num
    }

    nums[idx] = num

    res = append(res, sum_even)
  }

  return res
}
