package main

func longestCommonSubsequence(text1 string, text2 string) int {
	dp := make([][]int, len(text1))
	for i := range dp {
		dp[i] = make([]int, len(text2))
	}

	for i, c1 := range text1 {
		for j, c2 := range text2 {
			if i > 0 {
				dp[i][j] = max(dp[i][j], dp[i-1][j])
			}

			if j > 0 {
				dp[i][j] = max(dp[i][j], dp[i][j-1])
			}

			if c1 == c2 {
				if i == 0 || j == 0 {
					dp[i][j] = max(dp[i][j], 1)
				} else {
					dp[i][j] = max(dp[i][j], 1+dp[i-1][j-1])
				}
			}
		}
	}

	return dp[len(text1)-1][len(text2)-1]
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
