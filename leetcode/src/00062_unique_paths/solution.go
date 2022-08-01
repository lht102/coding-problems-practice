package leetcode

func uniquePaths(m int, n int) int {
	dp := [2][]int{}
	for i := 0; i < 2; i++ {
		dp[i] = make([]int, n)
		for j := 0; j < n; j++ {
			dp[i][j] = 1
		}
	}
	for i := 1; i < m; i++ {
		for j := 1; j < n; j++ {
			dp[i&1][j] = dp[(i-1)&1][j] + dp[i&1][j-1]
		}
	}
	return dp[(m-1)&1][n-1]
}
