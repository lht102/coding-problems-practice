package leetcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestNumFactoredBinaryTrees(t *testing.T) {
	tests := []struct {
		arr []int

		expected int
	}{
		{[]int{2, 4}, 3},
		{[]int{2, 4, 5, 10}, 7},
	}

	for i, tt := range tests {
		tt := tt
		t.Run(fmt.Sprintf("Test %d", i), func(t *testing.T) {
			t.Parallel()
			actual := numFactoredBinaryTrees(tt.arr)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
