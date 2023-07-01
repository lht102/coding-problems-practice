package leetcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestDistributeCookies(t *testing.T) {
	tests := []struct {
		cookies []int
		k       int

		expected int
	}{
		{[]int{8, 15, 10, 20, 8}, 2, 31},
		{[]int{6, 1, 3, 2, 2, 4, 1, 2}, 3, 7},
	}

	for i, tt := range tests {
		tt := tt
		t.Run(fmt.Sprintf("Test %d", i), func(t *testing.T) {
			t.Parallel()
			actual := distributeCookies(tt.cookies, tt.k)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
