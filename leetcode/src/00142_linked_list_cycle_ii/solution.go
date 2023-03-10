package leetcode

type ListNode struct {
	Val  int
	Next *ListNode
}

func detectCycle(head *ListNode) *ListNode {
	slow := head
	fast := head
	cycle := false
	for fast != nil && fast.Next != nil {
		slow = slow.Next
		fast = fast.Next.Next
		if slow == fast {
			cycle = true
			break
		}
	}
	if !cycle {
		return nil
	}
	res := head
	for res != slow {
		res = res.Next
		slow = slow.Next
	}
	return res
}
