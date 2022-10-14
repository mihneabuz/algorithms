package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func deleteMiddle(head *ListNode) *ListNode {
	head, _ = deleteRecursive(head, 0)
	return head
}

func deleteRecursive(head *ListNode, i int) (*ListNode, int) {
	if head == nil {
		return nil, i
	}

	next, length := deleteRecursive(head.Next, i+1)

	if i == length/2 {
		head = next
	} else {
		head.Next = next
	}

	return head, length
}
