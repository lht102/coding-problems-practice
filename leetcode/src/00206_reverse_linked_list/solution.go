package leetcode

type ListNode struct {
	Val  int
	Next *ListNode
}

func reverseList(head *ListNode) *ListNode {
	var newHead *ListNode
	for head != nil {
		tmp := head
		head = head.Next
		tmp.Next = newHead
		newHead = tmp
	}
	return newHead
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
