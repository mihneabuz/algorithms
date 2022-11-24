package main

func exist(board [][]byte, word string) bool {
	n, m := len(board), len(board[0])
	bytes := []byte(word)
	dirs := [][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}

	var search func(pos [2]int, visited [][2]int, remaining []byte) bool
	search = func(pos [2]int, visited [][2]int, remaining []byte) bool {
		if len(remaining) == 0 {
			return true
		}

		for _, dir := range dirs {
			x, y := pos[0]+dir[0], pos[1]+dir[1]
			if x >= 0 && y >= 0 && x < n && y < m {
				if board[x][y] != remaining[0] {
					continue
				}

				if alreadyVisited(x, y, visited) {
					continue
				}

				if search([2]int{x, y}, append(visited, pos), remaining[1:]) {
					return true
				}
			}
		}

		return false
	}

	for i, row := range board {
		for j, b := range row {
			if bytes[0] == b && search([2]int{i, j}, [][2]int{}, bytes[1:]) {
				return true
			}
		}
	}

	return false
}

func alreadyVisited(x, y int, visited [][2]int) bool {
	for _, vis := range visited {
		if x == vis[0] && y == vis[1] {
			return true
		}
	}

	return false
}
