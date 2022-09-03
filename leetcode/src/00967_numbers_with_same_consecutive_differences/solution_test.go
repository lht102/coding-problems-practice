package leetcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestNumsSameConsecDiff(t *testing.T) {
	tests := []struct {
		n int
		k int

		expected []int
	}{
		{3, 7, []int{181, 292, 707, 818, 929}},
		{2, 1, []int{10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89, 98}},
	}

	for i, tt := range tests {
		tt := tt
		t.Run(fmt.Sprintf("Test %d", i), func(t *testing.T) {
			t.Parallel()
			actual := numsSameConsecDiff(tt.n, tt.k)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
