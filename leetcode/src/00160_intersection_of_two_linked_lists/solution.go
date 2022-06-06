package leetcode

type ListNode struct {
	Val  int
	Next *ListNode
}

// nolint: unused, deadcode
func getIntersectionNode(headA, headB *ListNode) *ListNode {
	if headA == nil || headB == nil {
		return nil
	}

	p := headA
	q := headB

	for p != q {
		if p != nil {
			p = p.Next
		} else {
			p = headB
		}
		if q != nil {
			q = q.Next
		} else {
			q = headA
		}
	}

	return p
}
