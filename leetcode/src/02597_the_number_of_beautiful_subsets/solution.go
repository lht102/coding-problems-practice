package leetcode

func beautifulSubsets(nums []int, k int) int {
	return dfs(nums, k, 0, 0) - 1
}

func dfs(nums []int, k int, i int, mask int) int {
	if i == len(nums) {
		return 1
	}
	notTake := dfs(nums, k, i+1, mask)
	for j := 0; j < i; j++ {
		if mask&(1<<j) > 0 && abs(nums[j]-nums[i]) == k {
			return notTake
		}
	}
	take := dfs(nums, k, i+1, mask|(1<<i))
	return notTake + take
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}
