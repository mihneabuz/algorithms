package main

func validPath(n int, edges [][]int, source int, destination int) bool {
	if source == destination {
		return true
	}

	graph := make([][]int, n)
	for _, edge := range edges {
		graph[edge[0]] = append(graph[edge[0]], edge[1])
		graph[edge[1]] = append(graph[edge[1]], edge[0])
	}

	seen := make([]bool, n)
	queue := []int{source}

	for len(queue) > 0 {
		node := queue[len(queue)-1]
		queue = queue[:len(queue)-1]

		for _, neigh := range graph[node] {
			if neigh == destination {
				return true
			}

			if !seen[neigh] {
				seen[neigh] = true
				queue = append(queue, neigh)
			}
		}
	}

	return false
}
