package leetcode

func eventualSafeNodes(graph [][]int) []int {
	n := len(graph)
	colors := make([]int, n)
	var res []int
	for i := 0; i < n; i++ {
		if !dfs(graph, colors, i) {
			res = append(res, i)
		}
	}
	return res
}

func dfs(adj [][]int, colors []int, u int) bool {
	colors[u] = 1
	for _, v := range adj[u] {
		switch colors[v] {
		case 0:
			if dfs(adj, colors, v) {
				return true
			}
		case 1:
			return true
		}
	}
	colors[u] = 2
	return false
}
