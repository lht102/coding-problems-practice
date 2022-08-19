package leetcode

func isPossible(nums []int) bool {
	freq := map[int]int{}
	for _, num := range nums {
		freq[num] += 1
	}
	endFreq := map[int]int{}
	for _, num := range nums {
		if freq[num] == 0 {
			continue
		}
		freq[num] -= 1
		if endFreq[num-1] > 0 {
			endFreq[num-1]--
			endFreq[num]++
		} else if freq[num+1] > 0 && freq[num+2] > 0 {
			freq[num+1]--
			freq[num+2]--
			endFreq[num+2]++
		} else {
			return false
		}
	}
	return true
}
