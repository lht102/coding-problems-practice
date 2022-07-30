package leetcode

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func flatten(root *TreeNode) {
	dfs(root, nil)
}

func dfs(root *TreeNode, prev *TreeNode) *TreeNode {
	if root == nil {
		return prev
	}
	prev = dfs(root.Right, prev)
	prev = dfs(root.Left, prev)
	root.Right = prev
	root.Left = nil
	return root
}
