package leetcode

func movesToStamp(stamp string, target string) []int {
	s := []rune(stamp)
	t := []rune(target)
	var res []int
	for {
		isUpdated := false
		for i := 0; i < len(t)-len(s)+1; i++ {
			if canUpdate(s, t, i) {
				for j := 0; j < len(s); j++ {
					t[i+j] = '*'
				}
				res = append(res, i)
				isUpdated = true
			}
		}
		if !isUpdated {
			break
		}
	}
	for i := 0; i < len(t); i++ {
		if t[i] != '*' {
			return []int{}
		}
	}
	for i := len(res)/2 - 1; i >= 0; i-- {
		j := len(res) - 1 - i
		res[i], res[j] = res[j], res[i]
	}
	return res
}

func canUpdate(s []rune, t []rune, i int) bool {
	flag := false
	for j := 0; j < len(s); j++ {
		if t[i+j] == '*' {
			continue
		} else if t[i+j] != s[j] {
			return false
		} else {
			flag = true
		}
	}
	return flag
}
