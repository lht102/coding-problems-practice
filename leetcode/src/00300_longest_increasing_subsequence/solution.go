package leetcode

func lengthOfLIS(nums []int) int {
	dp := make([]int, len(nums))
	res := 0
	for i := 0; i < len(nums); i++ {
		res = max(res, solve(nums, i, dp))
	}
	return res
}

func solve(nums []int, idx int, dp []int) int {
	if idx == len(nums) {
		return 0
	}
	if dp[idx] != 0 {
		return dp[idx]
	}
	res := 1
	for i := idx + 1; i < len(nums); i++ {
		if nums[i] > nums[idx] {
			res = max(res, 1+solve(nums, i, dp))
		}
	}
	dp[idx] = res
	return res
}

func max(x int, y int) int {
	if x < y {
		return y
	}
	return x
}
