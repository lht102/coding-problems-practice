package leetcode

import "strings"

func reverseWords(s string) string {
	return join(reverse(filter(split(s, ' '), func(s string) bool { return len(s) > 0 })), " ")
}

func split(s string, delimiter rune) []string {
	var res []string
	arr := []rune(s)
	j := 0
	for i := 0; i < len(arr); i++ {
		if arr[i] == delimiter {
			res = append(res, string(arr[j:i]))
			j = i + 1
		}
	}
	if j == 0 {
		return []string{s}
	}
	return append(res, string(arr[j:]))
}

func filter(arr []string, predicate func(s string) bool) []string {
	var res []string
	for _, s := range arr {
		if predicate(s) {
			res = append(res, s)
		}
	}
	return res
}

func reverse(arr []string) []string {
	n := len(arr)
	for i := 0; i < n/2; i++ {
		j := n - 1 - i
		arr[i], arr[j] = arr[j], arr[i]
	}
	return arr
}

func join(arr []string, sep string) string {
	if len(arr) == 0 {
		return ""
	}
	var sb strings.Builder
	sb.WriteString(arr[0])
	for i := 1; i < len(arr); i++ {
		sb.WriteString(sep)
		sb.WriteString(arr[i])
	}
	return sb.String()
}
