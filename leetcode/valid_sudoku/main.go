package main

func isValidSudoku(board [][]byte) bool {
	for col, row := range board {
		if !valid(row...) {
			return false
		}

		if !valid(getCol(board, col)...) {
			return false
		}
	}

	for i := 0; i < 3; i += 1 {
		for j := 0; j < 3; j += 1 {
			if !valid(getSquare(board, i, j)...) {
				return false
			}
		}
	}

	return true
}

func getCol(board [][]byte, col int) []byte {
	res := make([]byte, 0, 9)
	for i := 0; i < 9; i += 1 {
		res = append(res, board[i][col])
	}
	return res
}

func getSquare(board [][]byte, i, j int) []byte {
	res := make([]byte, 0, 9)
	for ii := 0; ii < 3; ii += 1 {
		for jj := 0; jj < 3; jj += 1 {
			res = append(res, board[i*3+ii][j*3+jj])
		}
	}
	return res
}

func valid(bytes ...byte) bool {
	seen := make([]byte, 9)

	for _, b := range bytes {
		if b == 46 {
			continue
		}
		num := b - '1'

		if seen[num] > 0 {
			return false
		}
		seen[num] += 1
	}

	return true
}
