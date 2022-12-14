package main

func rob(nums []int) int {
	dp := make([]int, len(nums))
	dp[0] = nums[0]

	if len(nums) > 1 {
		dp[1] = max(nums[0], nums[1])
	}

	for i := 2; i < len(nums); i += 1 {
		dp[i] = max(dp[i-1], nums[i]+dp[i-2])
	}

	return dp[len(nums)-1]
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
