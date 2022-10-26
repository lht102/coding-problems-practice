package leetcode

func checkSubarraySum(nums []int, k int) bool {
	pSumIdx := map[int]int{
		0: -1,
	}
	sum := 0
	for i, num := range nums {
		sum += num
		if j, ok := pSumIdx[sum%k]; ok {
			if i-j > 1 {
				return true
			}
		} else {
			pSumIdx[sum%k] = i
		}
	}
	return false
}
