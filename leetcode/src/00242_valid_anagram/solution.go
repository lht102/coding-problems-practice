package leetcode

func isAnagram(s string, t string) bool {
	if len(s) != len(t) {
		return false
	}

	freq := map[rune]int{}
	for _, ch := range s {
		freq[ch] += 1
	}

	for _, ch := range t {
		if freq[ch] == 0 {
			return false
		}
		freq[ch] -= 1
	}

	return true
}
