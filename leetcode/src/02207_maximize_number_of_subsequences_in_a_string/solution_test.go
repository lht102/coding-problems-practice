package leetcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestMaximumSubsequenceCount(t *testing.T) {
	tests := []struct {
		text    string
		pattern string

		expected int64
	}{
		{"abdcdbc", "ac", 4},
		{"aabb", "ab", 6},
		{"aabaa", "aa", 10},
	}

	for i, tt := range tests {
		tt := tt
		t.Run(fmt.Sprintf("Test %d", i), func(t *testing.T) {
			t.Parallel()
			actual := maximumSubsequenceCount(tt.text, tt.pattern)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
