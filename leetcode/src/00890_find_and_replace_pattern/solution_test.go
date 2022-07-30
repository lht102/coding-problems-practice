package leetcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestFindAndReplacePattern(t *testing.T) {
	tests := []struct {
		words   []string
		pattern string

		expected []string
	}{
		{[]string{"abc", "deq", "mee", "aqq", "dkd", "ccc"}, "abb", []string{"mee", "aqq"}},
		{[]string{"a", "b", "c"}, "a", []string{"a", "b", "c"}},
	}

	for i, tt := range tests {
		tt := tt
		t.Run(fmt.Sprintf("Test %d", i), func(t *testing.T) {
			t.Parallel()
			actual := findAndReplacePattern(tt.words, tt.pattern)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
