package leetcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestNumSquares(t *testing.T) {
	tests := []struct {
		n int

		expected int
	}{
		{12, 3},
		{13, 2},
	}

	for i, tt := range tests {
		tt := tt
		t.Run(fmt.Sprintf("Test %d", i), func(t *testing.T) {
			t.Parallel()
			actual := numSquares(tt.n)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
