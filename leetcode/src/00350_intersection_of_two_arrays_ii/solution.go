package leetcode

import (
	"slices"
)

func intersect(nums1 []int, nums2 []int) []int {
	slices.Sort(nums1)
	slices.Sort(nums2)

	i := 0
	j := 0
	m := len(nums1)
	n := len(nums2)
	var res []int

	for i < m && j < n {
		if nums1[i] < nums2[j] {
			i++
		} else if nums1[i] > nums2[j] {
			j++
		} else {
			res = append(res, nums1[i])
			i++
			j++
		}
	}
	return res
}
