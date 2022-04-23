package leetcode

func twoSum(nums []int, target int) []int {
	hm := map[int]int{}
	for i, num := range nums {
		j, ok := hm[target-num]
		if ok {
			return []int{j, i}
		} else {
			hm[num] = i
		}
	}
	return []int{}
}
