package leetcode

type Iterator struct {
	arr    []int
	curIdx int
}

func (iter *Iterator) hasNext() bool {
	return iter.curIdx < len(iter.arr)
}

func (iter *Iterator) next() int {
	if iter.hasNext() {
		res := iter.arr[iter.curIdx]
		iter.curIdx++
		return res
	}
	return -1
}

type PeekingIterator struct {
	iter   *Iterator
	items  []int
	curIdx int
}

func Constructor(iter *Iterator) *PeekingIterator {
	return &PeekingIterator{
		iter: iter,
	}
}

func (this *PeekingIterator) hasNext() bool {
	return this.curIdx < len(this.items) || this.iter.hasNext()
}

func (this *PeekingIterator) next() int {
	if this.curIdx < len(this.items) {
		res := this.items[this.curIdx]
		this.curIdx++
		return res
	}
	if this.iter.hasNext() {
		return this.iter.next()
	}
	return -1
}

func (this *PeekingIterator) peek() int {
	if this.iter.hasNext() && this.curIdx == len(this.items) {
		this.items = append(this.items, this.iter.next())
	}
	return this.items[this.curIdx]
}
