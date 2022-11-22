package leetcode

func numSquares(n int) int {
	visited := map[int]struct{}{}
	q := []int{0}
	visited[0] = struct{}{}
	res := 0
	for len(q) != 0 {
		for i := len(q); i > 0; i-- {
			var front int
			front, q = q[0], q[1:]
			if front == n {
				return res
			}
			for i := 1; i*i <= n-front; i++ {
				newVal := front + i*i
				if _, ok := visited[newVal]; ok {
					continue
				}
				visited[newVal] = struct{}{}
				q = append(q, newVal)
			}
		}
		res++
	}
	return res
}
