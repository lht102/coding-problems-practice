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

func (this *NumArray) Update(index int, val int) {
	this.add(index, val-this.SumRange(index, index))
}

func (this *NumArray) SumRange(left int, right int) int {
	return this.prefixSum(right) - this.prefixSum(left-1)
}

func (this *NumArray) add(i int, val int) {
	j := i + 1
	for j < len(this.tree) {
		this.tree[j] += val
		j += lsb(j)
	}
}

func (this *NumArray) prefixSum(i int) int {
	j := i + 1
	sum := 0
	for j != 0 {
		sum += this.tree[j]
		j -= lsb(j)
	}
	return sum
}

func lsb(x int) int {
	return x & -x
}
