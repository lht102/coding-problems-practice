package leetcode

type Point struct {
	x int
	y int
}

func pacificAtlantic(heights [][]int) [][]int {
	m := len(heights)
	n := len(heights[0])
	pacific := []Point{}
	atlantic := []Point{}
	for i := 0; i < m; i++ {
		pacific = append(pacific, Point{x: i, y: 0})
		atlantic = append(atlantic, Point{x: i, y: n - 1})
	}
	for j := 0; j < n; j++ {
		pacific = append(pacific, Point{x: 0, y: j})
		atlantic = append(atlantic, Point{x: m - 1, y: j})
	}
	var res [][]int
	visited1 := bfs(heights, m, n, pacific)
	visited2 := bfs(heights, m, n, atlantic)
	for p := range visited1 {
		if _, exist := visited2[p]; exist {
			res = append(res, []int{p.x, p.y})
		}
	}
	return res
}

func bfs(heights [][]int, m int, n int, starts []Point) map[Point]struct{} {
	dirs := [5]int{-1, 0, 1, 0, -1}
	q := append([]Point(nil), starts...)
	visited := make(map[Point]struct{}, len(starts))
	for _, p := range starts {
		visited[p] = struct{}{}
	}
	for len(q) > 0 {
		var p Point
		p, q = q[0], q[1:]
		for i := 0; i < 4; i++ {
			np := Point{
				x: p.x + dirs[i],
				y: p.y + dirs[i+1],
			}
			if np.x >= m || np.x < 0 || np.y >= n || np.y < 0 {
				continue
			}
			if _, exist := visited[np]; exist {
				continue
			}
			if heights[np.x][np.y] < heights[p.x][p.y] {
				continue
			}
			q = append(q, np)
			visited[np] = struct{}{}
		}
	}
	return visited
}
