package leetcode

type ListNode struct {
	Val  int
	Next *ListNode
}

func reverseList(head *ListNode) *ListNode {
	var prev *ListNode
	cur := head
	for cur != nil {
		next := cur.Next
		cur.Next = prev
		prev = cur
		cur = next
	}
	return prev
}

func solve(head *ListNode) *ListNode {
	if head == nil || head.Next == nil {
		return head
	}
	newHead := solve(head.Next)
	head.Next.Next = head
	head.Next = nil
	return newHead
}
