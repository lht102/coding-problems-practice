package leetcode

import "strings"

func gcdOfStrings(str1 string, str2 string) string {
	for i := min(len(str1), len(str2)); i >= 1; i-- {
		if isValid(str1, str2, i) {
			return str1[:i]
		}
	}
	return ""
}

func isValid(str1 string, str2 string, k int) bool {
	if len(str1)%k > 0 || len(str2)%k > 0 {
		return false
	}
	prefix := str1[:k]
	return strings.ReplaceAll(str1, prefix, "") == "" && strings.ReplaceAll(str2, prefix, "") == ""
}

func min(x int, y int) int {
	if x < y {
		return x
	}
	return y
}
