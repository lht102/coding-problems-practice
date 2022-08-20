package leetcode

import "math"

func minRefuelStops(target int, startFuel int, stations [][]int) int {
	n := len(stations)
	dp := make([][]int, n+1)
	for i := 0; i <= n; i++ {
		dp[i] = make([]int, n+1)
		for j := 0; j <= n; j++ {
			dp[i][j] = -1
		}
	}
	for i := n; i >= 0; i-- {
		solve(n, i, startFuel, stations, dp)
	}
	for i := 0; i <= n; i++ {
		if dp[n][i] >= target {
			return i
		}
	}
	return -1
}

func solve(i int, remaingRefuelNum int, curFuel int, stations [][]int, dp [][]int) int {
	if remaingRefuelNum == 0 {
		dp[i][remaingRefuelNum] = curFuel
		return dp[i][remaingRefuelNum]
	}
	if remaingRefuelNum > i {
		dp[i][remaingRefuelNum] = math.MinInt32
		return dp[i][remaingRefuelNum]
	}
	if dp[i][remaingRefuelNum] != -1 {
		return dp[i][remaingRefuelNum]
	}
	notTake := solve(i-1, remaingRefuelNum, curFuel, stations, dp)
	take := solve(i-1, remaingRefuelNum-1, curFuel, stations, dp)
	dis := math.MinInt32
	if take >= stations[i-1][0] {
		dis = take + stations[i-1][1]
	}
	dp[i][remaingRefuelNum] = max(notTake, dis)
	return dp[i][remaingRefuelNum]
}

func max(x int, y int) int {
	if x > y {
		return x
	}
	return y
}
