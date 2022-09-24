package leetcode

func trap(height []int) int {
	dp1 := make([]int, len(height))
	dp1[0] = height[0]
	for i := 1; i < len(height); i++ {
		dp1[i] = max(dp1[i-1], height[i])
	}
	dp2 := make([]int, len(height))
	dp2[len(height)-1] = height[len(height)-1]
	for i := len(height) - 2; i >= 0; i-- {
		dp2[i] = max(dp2[i+1], height[i])
	}
	res := 0
	for i := 0; i < len(height); i++ {
		res += min(dp1[i], dp2[i]) - height[i]
	}
	return res
}

func max(x int, y int) int {
	if x < y {
		return y
	}
	return x
}

func min(x int, y int) int {
	if x < y {
		return x
	}
	return y
}
