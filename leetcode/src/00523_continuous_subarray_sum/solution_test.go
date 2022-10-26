package leetcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCheckSubarraySum(t *testing.T) {
	tests := []struct {
		nums []int
		k    int

		expected bool
	}{
		{[]int{23, 2, 4, 6, 7}, 6, true},
		{[]int{23, 2, 6, 4, 7}, 13, false},
		{[]int{5, 0, 0, 0}, 3, true},
	}

	for i, tt := range tests {
		tt := tt
		t.Run(fmt.Sprintf("Test %d", i), func(t *testing.T) {
			t.Parallel()
			actual := checkSubarraySum(tt.nums, tt.k)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
