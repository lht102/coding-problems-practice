package leetcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestUniquePaths(t *testing.T) {
	tests := []struct {
		m int
		n int

		expected int
	}{
		{3, 7, 28},
		{3, 2, 3},
	}

	for i, tt := range tests {
		tt := tt
		t.Run(fmt.Sprintf("Test %d", i), func(t *testing.T) {
			t.Parallel()
			actual := uniquePaths(tt.m, tt.n)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
