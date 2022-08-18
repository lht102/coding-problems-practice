package leetcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestMinSetSize(t *testing.T) {
	tests := []struct {
		arr []int

		expected int
	}{
		{[]int{3, 3, 3, 3, 5, 5, 5, 2, 2, 7}, 2},
		{[]int{7, 7, 7, 7, 7, 7}, 1},
	}

	for i, tt := range tests {
		tt := tt
		t.Run(fmt.Sprintf("Test %d", i), func(t *testing.T) {
			t.Parallel()
			actual := minSetSize(tt.arr)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
