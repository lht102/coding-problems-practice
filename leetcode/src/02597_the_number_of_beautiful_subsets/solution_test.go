package leetcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestBeautifulSubsets(t *testing.T) {
	tests := []struct {
		nums []int
		k    int

		expected int
	}{
		{[]int{2, 4, 6}, 2, 4},
		{[]int{1}, 1, 1},
	}

	for i, tt := range tests {
		tt := tt
		t.Run(fmt.Sprintf("Test %d", i), func(t *testing.T) {
			t.Parallel()
			actual := beautifulSubsets(tt.nums, tt.k)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
