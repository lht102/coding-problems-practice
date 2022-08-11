package leetcode

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isValidBST(root *TreeNode) bool {
	return dfs(root, nil, nil)
}

func dfs(root *TreeNode, leftBound *TreeNode, rightBound *TreeNode) bool {
	if root == nil {
		return true
	}
	if leftBound != nil {
		if root.Val <= leftBound.Val {
			return false
		}
	}
	if rightBound != nil {
		if root.Val >= rightBound.Val {
			return false
		}
	}
	return dfs(root.Left, leftBound, root) && dfs(root.Right, root, rightBound)
}
