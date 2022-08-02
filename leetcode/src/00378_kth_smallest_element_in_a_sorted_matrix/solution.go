package leetcode

func kthSmallest(matrix [][]int, k int) int {
	m := len(matrix)
	n := len(matrix[0])
	countLessOrEqualFn := func(mat [][]int, val int) int {
		j := n - 1
		cnt := 0
		for i := 0; i < m; i++ {
			for j >= 0 && mat[i][j] > val {
				j--
			}
			cnt += j + 1
		}
		return cnt
	}
	lo, hi := matrix[0][0], matrix[m-1][n-1]
	for lo < hi {
		mid := lo + (hi-lo)/2
		if countLessOrEqualFn(matrix, mid) >= k {
			hi = mid
		} else {
			lo = mid + 1
		}
	}
	return lo
}
