package leetcode

func largestOverlap(img1 [][]int, img2 [][]int) int {
	res := 0
	for i := 0; i < len(img1); i++ {
		for j := 0; j < len(img1[0]); j++ {
			res = max(res, overlapCount(i, j, img1, img2))
			res = max(res, overlapCount(i, j, img2, img1))
		}
	}
	return res
}

func overlapCount(xShift int, yShift int, img1 [][]int, img2 [][]int) int {
	leftShiftCnt := 0
	rightShiftCnt := 0
	ci := 0
	for i := yShift; i < len(img1); i++ {
		cj := 0
		for j := xShift; j < len(img1[0]); j++ {
			if img1[i][j] == 1 && img1[i][j] == img2[ci][cj] {
				leftShiftCnt++
			}
			if img1[i][cj] == 1 && img1[i][cj] == img2[ci][j] {
				rightShiftCnt++
			}
			cj++
		}
		ci++
	}
	return max(leftShiftCnt, rightShiftCnt)
}

func max(x int, y int) int {
	if x < y {
		return y
	}
	return x
}
