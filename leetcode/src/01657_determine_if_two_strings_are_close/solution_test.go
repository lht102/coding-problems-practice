package leetcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCloseStrings(t *testing.T) {
	tests := []struct {
		word1 string
		word2 string

		expected bool
	}{
		{"abc", "bca", true},
		{"a", "aa", false},
		{"cabbba", "abbccc", true},
	}

	for i, tt := range tests {
		tt := tt
		t.Run(fmt.Sprintf("Test %d", i), func(t *testing.T) {
			t.Parallel()
			actual := closeStrings(tt.word1, tt.word2)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
