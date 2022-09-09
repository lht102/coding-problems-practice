package leetcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestNumberOfWeakCharacters(t *testing.T) {
	tests := []struct {
		properties [][]int

		expected int
	}{
		{[][]int{{5, 5}, {6, 3}, {3, 6}}, 0},
		{[][]int{{2, 2}, {3, 3}}, 1},
		{[][]int{{1, 5}, {10, 4}, {4, 3}}, 1},
	}

	for i, tt := range tests {
		tt := tt
		t.Run(fmt.Sprintf("Test %d", i), func(t *testing.T) {
			t.Parallel()
			actual := numberOfWeakCharacters(tt.properties)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
