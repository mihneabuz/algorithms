package main

func nearestExit(maze [][]byte, entrance []int) int {
	dirs := [][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	m, n := len(maze), len(maze[0])

	queue := [][2]int{{entrance[0], entrance[1]}}
	maze[entrance[0]][entrance[1]] = '+'

	for steps := 0; len(queue) > 0; steps += 1 {
		next := make([][2]int, 0)

		for _, p := range queue {
			if isExit(p, n, m) && !(p[0] == entrance[0] && p[1] == entrance[1]) {
				return steps
			}

			for _, dir := range dirs {
				x, y := p[0]+dir[0], p[1]+dir[1]
				if x >= 0 && y >= 0 && x < m && y < n && maze[x][y] != '+' {
					next = append(next, [2]int{x, y})
					maze[x][y] = '+'
				}
			}
		}

		queue = next
	}

	return -1
}

func isExit(p [2]int, n, m int) bool {
	return p[0] == m-1 || p[1] == n-1 || p[0] == 0 || p[1] == 0
}
