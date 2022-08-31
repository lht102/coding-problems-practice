package leetcode

func numIslands(grid [][]byte) int {
	m := len(grid)
	n := len(grid[0])
	visited := make([][]bool, m)
	for i := 0; i < m; i++ {
		visited[i] = make([]bool, n)
	}
	res := 0
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			if grid[i][j] == '1' && !visited[i][j] {
				dfs(grid, visited, i, j)
				res++
			}
		}
	}
	return res
}

func dfs(grid [][]byte, visited [][]bool, i int, j int) bool {
	if i >= len(grid) || i < 0 || j >= len(grid[0]) || j < 0 || visited[i][j] || grid[i][j] != '1' {
		return false
	}
	visited[i][j] = true
	return dfs(grid, visited, i+1, j) ||
		dfs(grid, visited, i-1, j) ||
		dfs(grid, visited, i, j+1) ||
		dfs(grid, visited, i, j-1)
}
