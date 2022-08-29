package leetcode

func reorderedPowerOf2(n int) bool {
	arr := counter(n)
	for i := 0; i < 32; i++ {
		if arr == counter(1<<i) {
			return true
		}
	}
	return false
}

func counter(n int) [10]int {
	var res [10]int
	for n > 0 {
		res[n%10]++
		n /= 10
	}
	return res
}
