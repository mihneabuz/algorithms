package main

import "math"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func maxPathSum(root *TreeNode) int {
	_, longest := _maxPathSum(root)
	return longest
}

func _maxPathSum(root *TreeNode) (int, int) {
	if root == nil {
		return 0, math.MinInt
	}

	sumLeft, bestLeft := _maxPathSum(root.Left)
	sumRight, bestRight := _maxPathSum(root.Right)

	sumCurr := root.Val + max(0, max(sumLeft, sumRight))
	bestCurr := root.Val + max(sumLeft, 0) + max(sumRight, 0)

	return sumCurr, max(bestCurr, max(bestLeft, bestRight))
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
