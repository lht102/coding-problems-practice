package leetcode

func lengthOfLongestSubstring(s string) int {
	res := 0
	start := 0
	chIdx := map[rune]int{}
	for end, ch := range s {
		idx, ok := chIdx[ch]
		if ok {
			start = max(start, idx+1)
		}
		chIdx[ch] = end
		res = max(res, end+1-start)
	}
	return res
}

func max(x, y int) int {
	if x < y {
		return y
	}
	return x
}
