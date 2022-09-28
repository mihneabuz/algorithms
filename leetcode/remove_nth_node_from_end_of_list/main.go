package main

import "fmt"

type ListNode struct {
  Val int
  Next *ListNode
}

func removeNthFromEnd(head *ListNode, n int) *ListNode {
  len := removeNth(head, n)

  if n == len {
    return head.Next
  }

  return head
}

func removeNth(head *ListNode, n int) int {
  if head == nil {
    return 0
  }

  count := removeNth(head.Next, n)

  if n == count {
    head.Next = head.Next.Next
  }

  return 1 + count
}

func main() {
  input := &ListNode {
    Val: 1,
    Next: &ListNode {
      Val: 2,
      Next: &ListNode {
        Val: 3,
        Next: &ListNode {
          Val: 4,
          Next: &ListNode {
            Val: 5,
            Next: nil,
          },
        },
      },
    },
  }

  fmt.Println(removeNthFromEnd(input, 2));
}
