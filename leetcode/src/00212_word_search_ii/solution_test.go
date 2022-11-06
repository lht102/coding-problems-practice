package leetcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestFindWords(t *testing.T) {
	tests := []struct {
		board [][]byte
		words []string

		expected []string
	}{
		{
			[][]byte{
				{'o', 'a', 'a', 'n'},
				{'e', 't', 'a', 'e'},
				{'i', 'h', 'k', 'r'},
				{'i', 'f', 'l', 'v'},
			},
			[]string{"oath", "pea", "eat", "rain"},
			[]string{"oath", "eat"},
		},
		{
			[][]byte{
				{'a', 'b'},
				{'c', 'd'},
			},
			[]string{"abcb"},
			nil,
		},
		{
			[][]byte{
				{'a', 'a'},
			},
			[]string{"aaa"},
			nil,
		},
		{
			[][]byte{
				{'o', 'a', 'b', 'n'},
				{'o', 't', 'a', 'e'},
				{'a', 'h', 'k', 'r'},
				{'a', 'f', 'l', 'v'},
			},
			[]string{"oa", "oaa"},
			[]string{"oa", "oaa"},
		},
	}

	for i, tt := range tests {
		tt := tt
		t.Run(fmt.Sprintf("Test %d", i), func(t *testing.T) {
			t.Parallel()
			actual := findWords(tt.board, tt.words)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
