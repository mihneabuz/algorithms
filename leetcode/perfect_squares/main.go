package main

import "fmt"

func numSquares(n int) int {
	squares := make([]int, 0)
	for x := 2; x*x <= n; x += 1 {
		squares = append(squares, x*x)
	}

	dp := make([]int, n+1)
	dp[0] = 0
	dp[1] = 1

	for i := 2; i <= n; i += 1 {
		dp[i] = 1 + dp[i-1]
		for _, sq := range squares {
			if sq > i {
				break
			}

			dp[i] = min(dp[i], 1+dp[i-sq])
		}
	}

	return dp[n]
}

func min(x, y int) int {
	if x > y {
		return y
	}
	return x
}

func main() {
	fmt.Println(numSquares(12))
	fmt.Println(numSquares(13))
}
