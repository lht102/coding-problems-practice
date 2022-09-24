package leetcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPalindromePairs(t *testing.T) {
	tests := []struct {
		words []string

		expected [][]int
	}{
		{[]string{"abcd", "dcba", "lls", "s", "sssll"}, [][]int{{0, 1}, {1, 0}, {3, 2}, {2, 4}}},
		{[]string{"bat", "tab", "cat"}, [][]int{{0, 1}, {1, 0}}},
		{[]string{"a", ""}, [][]int{{0, 1}, {1, 0}}},
	}

	for i, tt := range tests {
		tt := tt
		t.Run(fmt.Sprintf("Test %d", i), func(t *testing.T) {
			t.Parallel()
			actual := palindromePairs(tt.words)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
