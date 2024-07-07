package leetcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestThreeConsecutiveOdds(t *testing.T) {
	tests := []struct {
		arr []int

		expected bool
	}{
		{[]int{2, 6, 4, 1}, false},
		{[]int{1, 2, 34, 3, 4, 5, 7, 23, 12}, true},
	}

	for i, tt := range tests {
		tt := tt
		t.Run(fmt.Sprintf("Test %d", i), func(t *testing.T) {
			t.Parallel()
			actual := threeConsecutiveOdds(tt.arr)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
