package leetcode

type Node struct {
	Val      int
	Children []*Node
}

func levelOrder(root *Node) [][]int {
	if root == nil {
		return nil
	}
	var res [][]int
	q := []*Node{root}
	for len(q) > 0 {
		n := len(q)
		var levelElements []int
		for i := 0; i < n; i++ {
			var p *Node
			p, q = q[0], q[1:]
			q = append(q, p.Children...)
			levelElements = append(levelElements, p.Val)
		}
		res = append(res, levelElements)
	}
	return res
}
