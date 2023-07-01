package leetcode

import "math"

func distributeCookies(cookies []int, k int) int {
	distributions := make([]int, k)
	return solve(cookies, distributions, 0)
}

func solve(cookies []int, distributions []int, i int) int {
	if i == len(cookies) {
		return maxSliceInt(distributions)
	}
	res := math.MaxInt
	for j := range distributions {
		distributions[j] += cookies[i]
		res = min(res, solve(cookies, distributions, i+1))
		distributions[j] -= cookies[i]
	}
	return res
}

func maxSliceInt(nums []int) int {
	res := 0
	for _, num := range nums {
		if num > res {
			res = num
		}
	}
	return res
}

func min(x int, y int) int {
	if x < y {
		return x
	}
	return y
}
