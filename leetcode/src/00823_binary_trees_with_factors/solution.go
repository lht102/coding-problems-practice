package leetcode

import "sort"

func numFactoredBinaryTrees(arr []int) int {
	const m = 1_000_000_007
	sort.SliceStable(arr, func(i, j int) bool {
		return arr[i] < arr[j]
	})
	dp := make(map[int]int, len(arr))
	dp[arr[0]] = 1
	for _, root := range arr {
		cnt := 1
		for firstChild := range dp {
			secondChild := root / firstChild
			_, found := dp[secondChild]
			if root%firstChild == 0 && found {
				cnt = (cnt + int(int64(dp[firstChild])*int64(dp[secondChild])%int64(m))) % m
			}
		}
		dp[root] = cnt
	}
	res := 0
	for _, cnt := range dp {
		res = (res + cnt) % m
	}
	return res
}
