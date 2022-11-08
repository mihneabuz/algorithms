package main

func makeGood(s string) string {
	b := []byte(s)
	i := 0
	for i < len(b)-1 {
		if b[i] == b[i+1]+32 || b[i]+32 == b[i+1] {
			b = append(b[:i], b[i+2:]...)
			if i > 0 {
				i -= 1
			}
		} else {
			i += 1
		}
	}
	return string(b)
}
