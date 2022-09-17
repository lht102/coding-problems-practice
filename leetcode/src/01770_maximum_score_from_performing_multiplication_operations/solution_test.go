package leetcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestTwoSum(t *testing.T) {
	tests := []struct {
		nums       []int
		mulipliers []int

		expected int
	}{
		{[]int{1, 2, 3}, []int{3, 2, 1}, 14},
		{[]int{-5, -3, -3, -2, 7, 1}, []int{-10, -5, 3, 4, 6}, 102},
	}

	for i, tt := range tests {
		tt := tt
		t.Run(fmt.Sprintf("Test %d", i), func(t *testing.T) {
			t.Parallel()
			actual := maximumScore(tt.nums, tt.mulipliers)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
