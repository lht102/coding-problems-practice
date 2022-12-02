package leetcode

import "sort"

func closeStrings(word1 string, word2 string) bool {
	freq1 := counter(word1)
	freq2 := counter(word2)
	set1 := counterKeys(freq1)
	set2 := counterKeys(freq2)
	cnt1 := counterValues(freq1)
	cnt2 := counterValues(freq2)
	return setRuneEqual(set1, set2) && sliceIntEqual(cnt1, cnt2)
}

func counter(word string) map[rune]int {
	freq := map[rune]int{}
	for _, ch := range word {
		freq[ch]++
	}
	return freq
}

func counterKeys(freq map[rune]int) map[rune]struct{} {
	res := make(map[rune]struct{}, len(freq))
	for ch := range freq {
		res[ch] = struct{}{}
	}
	return res
}

func counterValues(freq map[rune]int) []int {
	res := make([]int, 0, len(freq))
	for _, v := range freq {
		res = append(res, v)
	}
	sort.Ints(res)
	return res
}

func sliceIntEqual(s1 []int, s2 []int) bool {
	if len(s1) != len(s2) {
		return false
	}
	for i := range s1 {
		if s1[i] != s2[i] {
			return false
		}
	}
	return true
}

func setRuneEqual(s1 map[rune]struct{}, s2 map[rune]struct{}) bool {
	if len(s1) != len(s2) {
		return false
	}
	for k := range s1 {
		if _, ok := s2[k]; !ok {
			return false
		}
	}
	return true
}
