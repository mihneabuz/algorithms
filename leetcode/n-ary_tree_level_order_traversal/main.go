package main

import "fmt"

type Node struct {
  Val int
  Children []*Node
}

func levelOrder(root *Node) [][]int {
  var results [][]int

  if root == nil {
    return make([][]int, 0)
  }

  results = append(results, []int{root.Val})

  level := root.Children
  for len(level) > 0 {
    nextLevel   := make([]*Node, 0, len(level))
    levelValues := make([]int, 0, len(level))

    for _, node := range level {
      levelValues = append(levelValues, node.Val)
      nextLevel = append(nextLevel, node.Children...)
    }

    results = append(results, levelValues)
    level = nextLevel
  }

  return results
}

func main() {
  tree := Node{
    Val: 1,
    Children: []*Node{
      {
        Val: 3,
        Children: []*Node{
          {
            Val: 5,
            Children: []*Node{},
          },
          {
            Val: 6,
            Children: []*Node{},
          },
        },
      },
      {
        Val: 2,
        Children: []*Node{},
      },
      {
        Val: 4,
        Children: []*Node{},
      },
    },
  }

  fmt.Println(levelOrder(&tree))
}
