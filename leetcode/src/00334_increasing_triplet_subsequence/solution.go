package leetcode

import "math"

func increasingTriplet(nums []int) bool {
	x, y := math.MaxInt, math.MaxInt
	for _, num := range nums {
		if num < x {
			x = num
		}
		if num < y && num > x {
			y = num
		}
		if num > y {
			return true
		}
	}
	return false
}
