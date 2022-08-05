package leetcode

func combinationSum4(nums []int, target int) int {
	dp := make([]int, target+1)
	for i := 0; i < len(dp); i++ {
		dp[i] = -1
	}
	return solve(nums, target, dp)
}

func solve(nums []int, target int, dp []int) int {
	if target == 0 {
		return 1
	}
	if dp[target] != -1 {
		return dp[target]
	}
	res := 0
	for _, num := range nums {
		if num <= target {
			res += solve(nums, target-num, dp)
		}
	}
	dp[target] = res
	return dp[target]
}
