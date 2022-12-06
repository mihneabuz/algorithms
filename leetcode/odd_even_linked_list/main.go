package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func oddEvenList(head *ListNode) *ListNode {
  if head == nil {
    return nil
  }

  if head.Next == nil {
    return head
  }

	nodes := oddEven(head, true, [2][]*ListNode{make([]*ListNode, 0), make([]*ListNode, 0)})

	for i := 1; i < len(nodes[0]); i += 1 {
		nodes[0][i-1].Next = nodes[0][i]
	}
	nodes[0][len(nodes[0])-1].Next = nodes[1][0]

	for i := 1; i < len(nodes[1]); i += 1 {
		nodes[1][i-1].Next = nodes[1][i]
	}
  nodes[1][len(nodes[1])-1].Next = nil

	return nodes[0][0]
}

func oddEven(head *ListNode, odd bool, nodes [2][]*ListNode) [2][]*ListNode {
	if head == nil {
		return nodes
	}

	if odd {
		nodes[0] = append(nodes[0], head)
		return oddEven(head.Next, false, nodes)
	} else {
		nodes[1] = append(nodes[1], head)
		return oddEven(head.Next, true, nodes)
	}
}
