package leetcode

import "math"

func maximumScore(nums []int, multipliers []int) int {
	m := len(multipliers)
	dp := make([][]int, m)
	for i := range dp {
		dp[i] = make([]int, m)
		for j := range dp[i] {
			dp[i][j] = math.MinInt
		}
	}
	return solve(nums, multipliers, 0, 0, dp)
}

func solve(nums []int, mulitpliers []int, op int, left int, dp [][]int) int {
	if op == len(mulitpliers) {
		return 0
	}
	if dp[op][left] != math.MinInt {
		return dp[op][left]
	}
	dp[op][left] = max(
		mulitpliers[op]*nums[left]+solve(nums, mulitpliers, op+1, left+1, dp),
		mulitpliers[op]*nums[len(nums)-1-op+left]+solve(nums, mulitpliers, op+1, left, dp),
	)
	return dp[op][left]
}

func max(x int, y int) int {
	if x < y {
		return y
	}
	return x
}
