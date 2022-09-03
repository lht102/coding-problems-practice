package leetcode

func numsSameConsecDiff(n int, k int) []int {
	if n == 1 {
		return []int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
	}
	var res []int
	for i := 1; i < 10; i++ {
		dfs(n-1, k, i, &res)
	}
	return res
}

func dfs(n int, k int, num int, res *[]int) {
	if n == 0 {
		*res = append(*res, num)
		return
	}
	digit := num % 10
	path := []int{digit - k}
	if k != 0 {
		path = append(path, digit+k)
	}
	for _, nextDigit := range path {
		if nextDigit >= 0 && nextDigit < 10 {
			dfs(n-1, k, num*10+nextDigit, res)
		}
	}
}
