package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func leafSimilar(root1 *TreeNode, root2 *TreeNode) bool {
  leafs1, leafs2 := leafs(root1), leafs(root2)

  if len(leafs1) != len(leafs2) {
    return false
  }

  for i := range leafs1 {
    if leafs1[i] != leafs2[i] {
      return false
    }
  }

  return true
}

func leafs(root *TreeNode) []int {
  if root == nil {
    return nil
  }

  left := leafs(root.Left)
  right := leafs(root.Right)

  if right == nil && left == nil {
    return []int{root.Val}
  }

  res := make([]int, 0)
  if left != nil {
    res = append(res, left...)
  }
  if right != nil {
    res = append(res, right...)
  }

  return res
}
