package leetcode

func palindromePairs(words []string) [][]int {
	indexByReversedStrByLen := map[int]map[string]int{}
	for i := 0; i < len(words); i++ {
		m := len(words[i])
		_, ok := indexByReversedStrByLen[m]
		if !ok {
			indexByReversedStrByLen[m] = make(map[string]int)
		}
		indexByReversedStrByLen[m][reverse(words[i])] = i
	}
	var res [][]int
	for i := 0; i < len(words); i++ {
		w := words[i]
		m := len(w)
		for sz, indexByReversedStr := range indexByReversedStrByLen {
			if sz > m {
				continue
			}
			if sz == m {
				j, ok := indexByReversedStr[w]
				if ok && i != j {
					res = append(res, []int{i, j})
				}
				continue
			}
			if isPalindrome(w, sz, m-1) {
				j, ok := indexByReversedStr[w[:sz]]
				if ok {
					res = append(res, []int{i, j})
				}
			}
			if isPalindrome(w, 0, m-1-sz) {
				j, ok := indexByReversedStr[w[m-sz:]]
				if ok {
					res = append(res, []int{j, i})
				}
			}
		}
	}
	return res
}

func isPalindrome(s string, i int, j int) bool {
	for i < j && s[i] == s[j] {
		i++
		j--
	}
	return i >= j
}

func reverse(s string) string {
	runes := []rune(s)
	for i, j := 0, len(runes)-1; i < j; i, j = i+1, j-1 {
		runes[i], runes[j] = runes[j], runes[i]
	}
	return string(runes)
}
