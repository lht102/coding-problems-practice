package leetcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestLargestOverlap(t *testing.T) {
	tests := []struct {
		img1 [][]int
		img2 [][]int

		expected int
	}{
		{[][]int{{1, 1, 0}, {0, 1, 0}, {0, 1, 0}}, [][]int{{0, 0, 0}, {0, 1, 1}, {0, 0, 1}}, 3},
		{[][]int{{1}}, [][]int{{1}}, 1},
		{[][]int{{0}}, [][]int{{0}}, 0},
		{
			[][]int{
				{0, 0, 0, 0, 1},
				{0, 0, 0, 0, 0},
				{0, 0, 0, 0, 0},
				{0, 0, 0, 0, 0},
				{0, 0, 0, 0, 0},
			},
			[][]int{
				{0, 0, 0, 0, 0},
				{0, 0, 0, 0, 0},
				{0, 0, 0, 0, 0},
				{0, 0, 0, 0, 0},
				{1, 0, 0, 0, 0},
			},
			1,
		},
	}

	for i, tt := range tests {
		tt := tt
		t.Run(fmt.Sprintf("Test %d", i), func(t *testing.T) {
			t.Parallel()
			actual := largestOverlap(tt.img1, tt.img2)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
