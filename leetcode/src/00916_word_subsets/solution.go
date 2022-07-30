package leetcode

func wordSubsets(words1 []string, words2 []string) []string {
	counter := make(map[rune]int, 26)
	for _, w := range words2 {
		tmp := toCounter(w)
		for ch := 'a'; ch < 'z'; ch++ {
			counter[ch] = max(counter[ch], tmp[ch])
		}
	}
	var res []string
	for _, w := range words1 {
		tmp := toCounter(w)
		flag := true
		for ch := 'a'; ch < 'z'; ch++ {
			if tmp[ch] < counter[ch] {
				flag = false
				break
			}
		}
		if flag {
			res = append(res, w)
		}
	}
	return res
}

func toCounter(s string) map[rune]int {
	counter := make(map[rune]int, 26)
	for _, ch := range s {
		counter[ch] += 1
	}
	return counter
}

func max(x, y int) int {
	if x > y {
		return x
	}
	return y
}
