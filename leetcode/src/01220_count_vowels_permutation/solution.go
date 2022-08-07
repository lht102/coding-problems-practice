package leetcode

func countVowelPermutation(n int) int {
	const m = 1_000_000_007
	a, e, i, o, u := 0, 1, 2, 3, 4
	var dp [5][]int
	for i := 0; i < 5; i++ {
		dp[i] = make([]int, n+1)
		dp[i][1] = 1
	}
	for j := 2; j <= n; j++ {
		dp[a][j] = dp[e][j-1]
		dp[e][j] = (dp[a][j-1] + dp[i][j-1]) % m
		dp[i][j] = (dp[a][j-1] + dp[e][j-1] + dp[o][j-1] + dp[u][j-1]) % m
		dp[o][j] = (dp[i][j-1] + dp[u][j-1]) % m
		dp[u][j] = dp[a][j-1]
	}
	res := 0
	for _, ele := range dp {
		res = (res + ele[n]) % m
	}
	return res
}
