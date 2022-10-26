package leetcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestLengthOfLIS(t *testing.T) {
	tests := []struct {
		nums []int

		expected int
	}{
		{[]int{10, 9, 2, 5, 3, 7, 101, 18}, 4},
		{[]int{0, 1, 0, 3, 2, 3}, 4},
		{[]int{7, 7, 7, 7, 7, 7, 7}, 1},
	}

	for i, tt := range tests {
		tt := tt
		t.Run(fmt.Sprintf("Test %d", i), func(t *testing.T) {
			t.Parallel()
			actual := lengthOfLIS(tt.nums)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
