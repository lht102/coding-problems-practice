package leetcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestMinimumAverageDifference(t *testing.T) {
	tests := []struct {
		nums []int

		expected int
	}{
		{[]int{2, 5, 3, 9, 5, 3}, 3},
		{[]int{0}, 0},
	}

	for i, tt := range tests {
		tt := tt
		t.Run(fmt.Sprintf("Test %d", i), func(t *testing.T) {
			t.Parallel()
			actual := minimumAverageDifference(tt.nums)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
