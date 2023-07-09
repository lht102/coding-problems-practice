package leetcode

import "math"

func minSubArrayLen(target int, nums []int) int {
	res := math.MaxInt
	sum := 0
	left := 0
	for right := 0; right < len(nums); right++ {
		sum += nums[right]
		for sum >= target {
			res = min(res, right-left+1)
			sum -= nums[left]
			left++
		}
	}
	if res == math.MaxInt {
		return 0
	}
	return res
}

func min(x int, y int) int {
	if x < y {
		return x
	}
	return y
}
