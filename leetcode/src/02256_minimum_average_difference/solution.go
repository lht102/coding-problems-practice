package leetcode

import "math"

func minimumAverageDifference(nums []int) int {
	psum := prefixSum(nums)
	curMinAverage := int64(math.MaxInt64)
	idx := -1
	for i := range psum {
		diff := difference(psum, i)
		if idx == -1 || diff < curMinAverage {
			curMinAverage = diff
			idx = i
		}
	}
	return idx
}

func prefixSum(nums []int) []int64 {
	res := make([]int64, len(nums))
	res[0] = int64(nums[0])
	for i := 1; i < len(nums); i++ {
		res[i] += int64(nums[i]) + res[i-1]
	}
	return res
}

func difference(psum []int64, idx int) int64 {
	if idx == len(psum)-1 {
		return psum[len(psum)-1] / int64(len(psum))
	}
	return int64(math.Abs(float64(psum[idx]/int64(idx+1) - (psum[len(psum)-1]-psum[idx])/int64(len(psum)-1-idx))))
}
