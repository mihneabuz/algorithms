package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func maxProduct(root *TreeNode) int {
  total := _sum(root)
  _, res := _maxProduct(root, total)
  return res % 1_000_000_007
}

func _maxProduct(root *TreeNode, total int) (int, int) {
  if root == nil {
    return 0, 0
  }

  sumLeft, maxLeft := _maxProduct(root.Left, total)
  sumRight, maxRight := _maxProduct(root.Right, total)

  sumCurr := root.Val + sumLeft + sumRight
  maxCurr := max((total - sumLeft) * sumLeft, (total - sumRight) * sumRight)

  maxChild := max(maxLeft, maxRight)

  return sumCurr, max(maxCurr, maxChild)
}

func _sum(root *TreeNode) int {
  if root == nil {
    return 0
  }

  return root.Val + _sum(root.Left) + _sum(root.Right)
}

func max(a, b int) int {
  if a > b {
    return a
  }
  return b
}
