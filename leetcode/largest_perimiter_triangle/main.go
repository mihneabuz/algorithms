package main

import (
	"fmt"
	"sort"
)

func largestPerimeter(nums []int) int {
	sort.Ints(nums)

	for i := len(nums) - 3; i >= 0; i-- {
		if nums[i]+nums[i+1] > nums[i+2] {
			return nums[i] + nums[i+1] + nums[i+2]
		}
	}

	return 0
}

func main() {
  fmt.Println(largestPerimeter([]int{2,1,2}))
  fmt.Println(largestPerimeter([]int{1,2,1}))
}
