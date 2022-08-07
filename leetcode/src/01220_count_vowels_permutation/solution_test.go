package leetcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCountVowelPermutation(t *testing.T) {
	tests := []struct {
		n int

		expected int
	}{
		{1, 5},
		{2, 10},
		{5, 68},
	}

	for i, tt := range tests {
		tt := tt
		t.Run(fmt.Sprintf("Test %d", i), func(t *testing.T) {
			t.Parallel()
			actual := countVowelPermutation(tt.n)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
