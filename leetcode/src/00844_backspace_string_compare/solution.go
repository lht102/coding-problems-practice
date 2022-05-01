package leetcode

func backspaceCompare(s string, t string) bool {
	s1 := []rune(s)
	s2 := []rune(t)
	i := len(s1) - 1
	j := len(s2) - 1
	for {
		b1 := 0
		for i >= 0 && (b1 > 0 || s1[i] == '#') {
			if s1[i] == '#' {
				b1++
			} else {
				b1--
			}
			i--
		}
		b2 := 0
		for j >= 0 && (b2 > 0 || s2[j] == '#') {
			if s2[j] == '#' {
				b2++
			} else {
				b2--
			}
			j--
		}
		if i >= 0 && j >= 0 && s1[i] == s2[j] {
			i--
			j--
		} else {
			break
		}
	}
	return i == -1 && j == -1
}
