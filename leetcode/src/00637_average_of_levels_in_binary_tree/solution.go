package leetcode

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func averageOfLevels(root *TreeNode) []float64 {
	var path [][2]float64
	dfs(root, 0, &path)
	res := make([]float64, 0, len(path))
	for _, p := range path {
		res = append(res, p[0]/p[1])
	}
	return res
}

func dfs(root *TreeNode, level int, path *[][2]float64) {
	if root == nil {
		return
	}
	if level == len(*path) {
		*path = append(*path, [2]float64{})
	}
	(*path)[level][0] += float64(root.Val)
	(*path)[level][1] += 1.0
	dfs(root.Left, level+1, path)
	dfs(root.Right, level+1, path)
}
