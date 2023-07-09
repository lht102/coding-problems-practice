package leetcode

func singleNumber(nums []int) int {
	res := 0
	for i := 0; i < 64; i++ {
		cnt := 0
		mask := 1 << i
		for _, num := range nums {
			if num&mask != 0 {
				cnt++
			}
		}
		if cnt > 0 && cnt%3 != 0 {
			res |= mask
		}
	}
	return res
}
