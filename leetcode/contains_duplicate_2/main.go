package main

import "fmt"

func containsNearbyDuplicate(nums []int, k int) bool {
	seen := make(map[int]int)

	for i, x := range nums {
		j, found := seen[x]

		if found && i-j <= k {
			return true
		}

		seen[x] = i
	}

	return false
}

func main() {
	fmt.Println(containsNearbyDuplicate([]int{1, 2, 3, 1}, 3))
	fmt.Println(containsNearbyDuplicate([]int{1, 0, 1, 1}, 1))
	fmt.Println(containsNearbyDuplicate([]int{1, 2, 3, 1, 2, 3}, 2))
}
