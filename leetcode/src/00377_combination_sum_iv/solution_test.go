package leetcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCombinationSum4(t *testing.T) {
	tests := []struct {
		nums   []int
		target int

		expected int
	}{
		{[]int{1, 2, 3}, 4, 7},
		{[]int{9}, 3, 0},
	}

	for i, tt := range tests {
		tt := tt
		t.Run(fmt.Sprintf("Test %d", i), func(t *testing.T) {
			t.Parallel()
			actual := combinationSum4(tt.nums, tt.target)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
