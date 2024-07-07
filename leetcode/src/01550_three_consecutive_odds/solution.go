package leetcode

func threeConsecutiveOdds(arr []int) bool {
	cnt := 0
	for _, num := range arr {
		if num&1 == 1 {
			cnt++
		} else {
			cnt = 0
		}
		if cnt == 3 {
			return true
		}
	}
	return false
}
