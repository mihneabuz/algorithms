package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func (root *TreeNode) IsLeaf() bool {
  return root.Left == nil && root.Right == nil
}

func pseudoPalindromicPaths (root *TreeNode) int {
  return dfs(root, make([]int, 10));
}

func dfs(root *TreeNode, path []int) int {
  if root == nil {
    return 0
  }

  path[root.Val] += 1
  defer func() { path[root.Val] -= 1 }()

  if root.IsLeaf() {
    if isPseudoPalindrome(path) {
      return 1
    } else {
      return 0
    }

  } else {
    res := dfs(root.Left, path) + dfs(root.Right, path)

    return res
  }
}

func isPseudoPalindrome(path []int) bool {
  found_odd := false

  for _, count := range path {
    if count % 2 == 1 {
      if !found_odd {
        found_odd = true
      } else {
        return false
      }
    }
  }

  return true
}


func main() {
	tree := TreeNode{
		Val: 2,
		Left: &TreeNode{
			Val: 3,
			Left: &TreeNode{
				Val:   3,
				Left:  nil,
				Right: nil,
			},
			Right: &TreeNode{
				Val:   1,
				Left:  nil,
				Right: nil,
			},
		},
		Right: &TreeNode{
			Val: 1,
			Left: nil,
			Right: &TreeNode{
				Val:   1,
				Left:  nil,
				Right: nil,
			},
		},
	}

	fmt.Println(pseudoPalindromicPaths(&tree))
}
