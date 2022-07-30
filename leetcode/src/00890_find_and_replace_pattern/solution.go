package leetcode

import "strings"

func findAndReplacePattern(words []string, pattern string) []string {
	var res []string
	p := toPattern(pattern)
	for _, w := range words {
		if p == toPattern(w) {
			res = append(res, w)
		}
	}

	return res
}

func toPattern(s string) string {
	cnts := map[rune]int{}
	for _, ch := range s {
		cnts[ch] = len(cnts)
	}

	var sb strings.Builder
	for _, ch := range s {
		sb.WriteRune('a' + rune(cnts[ch]))
	}

	return sb.String()
}
