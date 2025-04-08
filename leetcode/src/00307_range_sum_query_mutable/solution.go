package leetcode

type NumArray struct {
	tree []int
}

func Constructor(nums []int) NumArray {
	arr := append([]int{0}, nums...)
	for i := 0; i < len(arr); i++ {
		p := i + lsb(i)
		if p < len(arr) {
			arr[p] += arr[i]
		}
	}
	return NumArray{
		tree: arr,
	}
}

func (na *NumArray) Update(index int, val int) {
	na.add(index, val-na.SumRange(index, index))
}

func (na *NumArray) SumRange(left int, right int) int {
	return na.prefixSum(right) - na.prefixSum(left-1)
}

func (na *NumArray) add(i int, val int) {
	j := i + 1
	for j < len(na.tree) {
		na.tree[j] += val
		j += lsb(j)
	}
}

func (na *NumArray) prefixSum(i int) int {
	j := i + 1
	sum := 0
	for j != 0 {
		sum += na.tree[j]
		j -= lsb(j)
	}
	return sum
}

func lsb(x int) int {
	return x & -x
}
