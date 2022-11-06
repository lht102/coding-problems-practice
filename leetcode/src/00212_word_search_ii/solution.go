package leetcode

func findWords(board [][]byte, words []string) []string {
	var res []string
	for _, w := range words {
		if search(board, w) {
			res = append(res, w)
		}
	}
	return res
}

func search(board [][]byte, word string) bool {
	for i := 0; i < len(board); i++ {
		for j := 0; j < len(board[0]); j++ {
			if dfs(board, i, j, word, 0) {
				return true
			}
		}
	}
	return false
}

func dfs(board [][]byte, ci int, cj int, word string, idx int) bool {
	if ci < 0 || ci >= len(board) || cj < 0 || cj >= len(board[0]) || word[idx] != board[ci][cj] {
		return false
	}
	if idx == len(word)-1 {
		return true
	}
	tmp := board[ci][cj]
	board[ci][cj] = '1'
	found := dfs(board, ci+1, cj, word, idx+1) ||
		dfs(board, ci-1, cj, word, idx+1) ||
		dfs(board, ci, cj+1, word, idx+1) ||
		dfs(board, ci, cj-1, word, idx+1)
	board[ci][cj] = tmp
	return found
}
