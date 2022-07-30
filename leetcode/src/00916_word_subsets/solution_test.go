package leetcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestWordSubsets(t *testing.T) {
	tests := []struct {
		words1 []string
		words2 []string

		expected []string
	}{
		{[]string{"amazon", "apple", "facebook", "google", "leetcode"}, []string{"e", "o"}, []string{"facebook", "google", "leetcode"}},
		{[]string{"amazon", "apple", "facebook", "google", "leetcode"}, []string{"l", "e"}, []string{"apple", "google", "leetcode"}},
	}

	for i, tt := range tests {
		tt := tt
		t.Run(fmt.Sprintf("Test %d", i), func(t *testing.T) {
			t.Parallel()
			actual := wordSubsets(tt.words1, tt.words2)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
