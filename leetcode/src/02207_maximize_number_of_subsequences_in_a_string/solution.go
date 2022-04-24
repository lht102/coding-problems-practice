package leetcode

func maximumSubsequenceCount(text string, pattern string) int64 {
	t := []rune(text)
	p := []rune(pattern)
	cnt2 := int64(0)
	for _, ch := range t {
		if ch == p[1] {
			cnt2++
		}
	}
	sum := int64(0)
	curSuffix := cnt2
	for _, ch := range t {
		if ch == p[1] {
			curSuffix--
		}
		if ch == p[0] {
			sum += curSuffix
		}
	}
	cnt1 := int64(0)
	extra := cnt2
	for _, ch := range t {
		if ch == p[0] {
			cnt1++
		}
		if ch == p[1] {
			cnt2--
		}
		extra = max(extra, cnt1, cnt2)
	}
	return sum + extra
}

func max(nums ...int64) int64 {
	res := nums[0]
	for _, n := range nums {
		if res < n {
			res = n
		}
	}
	return res
}
