package leetcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestIsPossible(t *testing.T) {
	tests := []struct {
		nums []int

		expected bool
	}{
		{[]int{1, 2, 3, 3, 4, 5}, true},
		{[]int{1, 2, 3, 3, 4, 4, 5, 5}, true},
		{[]int{1, 2, 3, 4, 4, 5}, false},
		{[]int{1, 2, 3, 4, 5, 5, 6, 7}, true},
		{[]int{3, 4, 4, 5, 6, 7, 8, 9, 10, 11}, false},
	}

	for i, tt := range tests {
		tt := tt
		t.Run(fmt.Sprintf("Test %d", i), func(t *testing.T) {
			t.Parallel()
			actual := isPossible(tt.nums)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
