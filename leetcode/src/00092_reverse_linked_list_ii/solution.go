package leetcode

type ListNode struct {
	Val  int
	Next *ListNode
}

func reverseBetween(head *ListNode, left int, right int) *ListNode {
	dummy := &ListNode{
		Val:  0,
		Next: head,
	}

	prev := dummy
	for i := 1; i < left; i++ {
		prev = prev.Next
	}

	cur := prev.Next
	for i := 0; i < right-left; i++ {
		next := cur.Next
		cur.Next = next.Next
		next.Next = cur
		prev.Next = next
		cur = cur.Next
		prev = prev.Next
	}

	return dummy.Next
}
