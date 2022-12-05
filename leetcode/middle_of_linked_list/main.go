package main

type ListNode struct {
    Val int
    Next *ListNode
}

func middleNode(head *ListNode) *ListNode {
    return middle(head, head)
}

func middle(slow, fast *ListNode) *ListNode {
    if fast == nil || fast.Next == nil {
        return slow
    }

    return middle(slow.Next, fast.Next.Next)
}
