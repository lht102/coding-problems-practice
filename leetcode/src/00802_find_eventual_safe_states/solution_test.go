package leetcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestEventualSafeNodes(t *testing.T) {
	tests := []struct {
		graph [][]int

		expected []int
	}{
		{[][]int{{1, 2}, {2, 3}, {5}, {0}, {5}, {}, {}}, []int{2, 4, 5, 6}},
		{[][]int{{1, 2, 3, 4}, {1, 2}, {3, 4}, {0, 4}, {}}, []int{4}},
	}

	for i, tt := range tests {
		tt := tt
		t.Run(fmt.Sprintf("Test %d", i), func(t *testing.T) {
			t.Parallel()
			actual := eventualSafeNodes(tt.graph)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
