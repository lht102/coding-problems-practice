package leetcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestFindSubstring(t *testing.T) {
	tests := []struct {
		s     string
		words []string

		expected []int
	}{
		{"barfoothefoobarman", []string{"foo", "bar"}, []int{0, 9}},
		{"wordgoodgoodgoodbestword", []string{"word", "good", "best", "word"}, []int{}},
		{"barfoofoobarthefoobarman", []string{"bar", "foo", "the"}, []int{6, 9, 12}},
		{"wordgoodgoodgoodbestword", []string{"word", "good", "best", "good"}, []int{8}},
	}

	for i, tt := range tests {
		tt := tt
		t.Run(fmt.Sprintf("Test %d", i), func(t *testing.T) {
			t.Parallel()
			actual := findSubstring(tt.s, tt.words)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
