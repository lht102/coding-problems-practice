package leetcode

import "sort"

func diagonalSort(mat [][]int) [][]int {
	m := len(mat)
	n := len(mat[0])
	diagonals := map[int][]int{}
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			diagonals[i-j] = append(diagonals[i-j], mat[i][j])
		}
	}
	for _, v := range diagonals {
		sort.SliceStable(v, func(i, j int) bool {
			return v[i] > v[j]
		})
	}
	res := make([][]int, m)
	for i := 0; i < m; i++ {
		res[i] = make([]int, n)
	}
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			key := i - j
			res[i][j], diagonals[key] = diagonals[key][len(diagonals[key])-1], diagonals[key][:len(diagonals[key])-1]
		}
	}
	return res
}
