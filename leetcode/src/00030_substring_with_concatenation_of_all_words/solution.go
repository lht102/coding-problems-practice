package leetcode

func findSubstring(s string, words []string) []int {
	wordLen := len(words[0])
	totalLen := wordLen * len(words)
	freq := map[string]int{}
	for _, w := range words {
		freq[w] += 1
	}
	res := []int{}
	i := 0
	for i+totalLen <= len(s) {
		freqClone := make(map[string]int, len(freq))
		for k, v := range freq {
			freqClone[k] = v
		}
		if isValid(freqClone, s[i:i+totalLen], wordLen) {
			res = append(res, i)
		}
		i++
	}
	return res
}

func isValid(freq map[string]int, s string, wordLen int) bool {
	for i := 0; i < len(s); i += wordLen {
		w := s[i : i+wordLen]
		freq[w]--
		if freq[w] < 0 {
			return false
		}
	}
	return true
}
