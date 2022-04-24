package leetcode

import (
	"sort"
)

func search(nums []int, target int) int {
	lo := 0
	hi := len(nums) - 1
	for lo < hi {
		mid := (hi-lo)/2 + lo
		if nums[mid] < nums[hi] {
			hi = mid
		} else {
			lo = mid + 1
		}
	}

	if target == nums[lo] {
		return lo
	}
	start := 0
	if target <= nums[len(nums)-1] {
		start = lo
	}
	end := len(nums)
	if target > nums[len(nums)-1] {
		end = lo
	}
	res := sort.SearchInts(nums[start:end], target)
	if res+start < len(nums) && nums[res+start] == target {
		return res + start
	}
	return -1
}
