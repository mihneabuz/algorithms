package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func pruneTree(root *TreeNode) *TreeNode {
	if dfs(root) == 0 {
		return nil
	} else {
		return root
	}
}

func dfs(root *TreeNode) int {
	if root == nil {
		return 0
	}

	left := dfs(root.Left)
	if left == 0 {
		root.Left = nil
	}

	right := dfs(root.Right)
	if right == 0 {
		root.Right = nil
	}

	return root.Val + left + right
}

func main() {
	tree := TreeNode{
		Val: 1,
		Left: &TreeNode{
			Val: 0,
			Left: &TreeNode{
				Val:   0,
				Left:  nil,
				Right: nil,
			},
			Right: &TreeNode{
				Val:   0,
				Left:  nil,
				Right: nil,
			},
		},
		Right: &TreeNode{
			Val: 1,
			Left: &TreeNode{
				Val:   0,
				Left:  nil,
				Right: nil,
			},
			Right: &TreeNode{
				Val:   1,
				Left:  nil,
				Right: nil,
			},
		},
	}

	fmt.Println(pruneTree(&tree))
}
