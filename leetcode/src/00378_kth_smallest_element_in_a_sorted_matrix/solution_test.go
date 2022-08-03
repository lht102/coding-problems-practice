package leetcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestKthSmallest(t *testing.T) {
	tests := []struct {
		matrix [][]int
		k      int

		expected int
	}{
		{[][]int{{1, 5, 9}, {10, 11, 13}, {12, 13, 15}}, 8, 13},
		{[][]int{{-5}}, 1, -5},
	}

	for i, tt := range tests {
		tt := tt
		t.Run(fmt.Sprintf("Test %d", i), func(t *testing.T) {
			t.Parallel()
			actual := kthSmallest(tt.matrix, tt.k)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
