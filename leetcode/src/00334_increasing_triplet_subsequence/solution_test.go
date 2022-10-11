package leetcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestTwoSum(t *testing.T) {
	tests := []struct {
		nums []int

		expected bool
	}{
		{[]int{1, 2, 3, 4, 5}, true},
		{[]int{5, 4, 3, 2, 1}, false},
		{[]int{2, 1, 5, 0, 4, 6}, true},
		{[]int{20, 100, 10, 12, 5, 13}, true},
		{[]int{0, 4, 2, 1, 0, -1, -3}, false},
	}

	for i, tt := range tests {
		tt := tt
		t.Run(fmt.Sprintf("Test %d", i), func(t *testing.T) {
			t.Parallel()
			actual := increasingTriplet(tt.nums)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
