package leetcode

func minSetSize(arr []int) int {
	var freq [100001]int
	for _, num := range arr {
		freq[num] += 1
	}
	maxFreq := 0
	for _, num := range arr {
		if freq[num] > maxFreq {
			maxFreq = freq[num]
		}
	}
	var buckets [100001]int
	for i := 0; i < 100001; i++ {
		buckets[freq[i]]++
	}
	remaining := len(arr) / 2
	res := 0
	for freq := maxFreq; remaining > 0; freq-- {
		if buckets[freq] == 0 {
			continue
		}
		inc := 0
		if remaining%freq > 0 {
			inc = 1
		}
		cnt := min(buckets[freq], remaining/freq+inc)
		remaining -= cnt * freq
		res += cnt
	}
	return res
}

func min(x int, y int) int {
	if x < y {
		return x
	}
	return y
}
