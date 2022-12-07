package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func rangeSumBST(root *TreeNode, low int, high int) int {
	if root == nil {
		return 0
	}

  res := 0

  if root.Val > low {
    res += rangeSumBST(root.Left, low, high);
  }

  if root.Val < high {
    res += rangeSumBST(root.Right, low, high);
  }

  if root.Val >= low && root.Val <= high {
    res += root.Val;
  }

	return res
}
