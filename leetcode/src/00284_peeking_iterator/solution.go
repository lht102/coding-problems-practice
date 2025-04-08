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

func (pi *PeekingIterator) hasNext() bool {
	return pi.curIdx < len(pi.items) || pi.iter.hasNext()
}

func (pi *PeekingIterator) next() int {
	if pi.curIdx < len(pi.items) {
		res := pi.items[pi.curIdx]
		pi.curIdx++
		return res
	}
	if pi.iter.hasNext() {
		return pi.iter.next()
	}
	return -1
}

func (pi *PeekingIterator) peek() int {
	if pi.iter.hasNext() && pi.curIdx == len(pi.items) {
		pi.items = append(pi.items, pi.iter.next())
	}
	return pi.items[pi.curIdx]
}
