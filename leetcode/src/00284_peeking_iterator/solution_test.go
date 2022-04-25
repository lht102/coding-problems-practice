package leetcode

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPeekIterator(t *testing.T) {
	peekingIterator := Constructor(&Iterator{arr: []int{1, 2, 3}})
	assert.Equal(t, 1, peekingIterator.next())
	assert.Equal(t, 2, peekingIterator.peek())
	assert.Equal(t, 2, peekingIterator.next())
	assert.Equal(t, 3, peekingIterator.next())
	assert.Equal(t, false, peekingIterator.hasNext())
}
