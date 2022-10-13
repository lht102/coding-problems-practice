package leetcode

type ListNode struct {
	Val  int
	Next *ListNode
}

func deleteNode(node *ListNode) {
	nextNode := node.Next
	node.Val = nextNode.Val
	node.Next = nextNode.Next
	nextNode.Next = nil
}
