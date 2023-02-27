package leetcode

type Node struct {
	Val         bool
	IsLeaf      bool
	TopLeft     *Node
	TopRight    *Node
	BottomLeft  *Node
	BottomRight *Node
}

func construct(grid [][]int) *Node {
	return solve(grid, 0, 0, len(grid))
}

func solve(grid [][]int, ci int, cj int, n int) *Node {
	if n == 1 {
		return &Node{
			Val:    grid[ci][cj] == 1,
			IsLeaf: true,
		}
	}

	m := n / 2
	topLeft := solve(grid, ci, cj, m)
	topRight := solve(grid, ci, cj+m, m)
	bottomLeft := solve(grid, ci+m, cj, m)
	bottomRight := solve(grid, ci+m, cj+m, m)

	if topLeft.IsLeaf && topRight.IsLeaf && bottomLeft.IsLeaf && bottomRight.IsLeaf &&
		topLeft.Val == topRight.Val && topRight.Val == bottomLeft.Val && bottomLeft.Val == bottomRight.Val {
		return &Node{
			Val:    topLeft.Val,
			IsLeaf: true,
		}
	}

	return &Node{
		TopLeft:     topLeft,
		TopRight:    topRight,
		BottomLeft:  bottomLeft,
		BottomRight: bottomRight,
	}
}
