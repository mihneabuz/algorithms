package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func maxAncestorDiff(root *TreeNode) int {
	return maxDiff(root, root.Val, root.Val)
}

func maxDiff(root *TreeNode, mn, mx int) int {
	if root == nil {
		return 0
	}

	curr := max(abs(mn-root.Val), abs(mx-root.Val))

	mn = min(mn, root.Val)
	mx = max(mx, root.Val)

	return max(curr, max(maxDiff(root.Left, mn, mx), maxDiff(root.Right, mn, mx)))
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}
