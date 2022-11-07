package main

func reverseVowels(s string) string {
	bytes := []byte(s)
	l, r := 0, len(bytes)-1

	for l < r {
		switch pack(isVowel(bytes[l]), isVowel(bytes[r])) {
		case pack(true, true):
			bytes[l], bytes[r] = bytes[r], bytes[l]
			l += 1
			r -= 1

		case pack(true, false):
			r -= 1

		case pack(false, true):
			l += 1

		default:
			l += 1
			r -= 1
		}
	}

	return string(bytes)
}

var vowels = map[byte]struct{}{
	'a': {}, 'e': {}, 'i': {}, 'o': {}, 'u': {},
	'A': {}, 'E': {}, 'I': {}, 'O': {}, 'U': {},
}

func isVowel(c byte) bool {
	_, r := vowels[c]
	return r
}

func pack(a, b bool) int {
	return bint(a)*2 + bint(b)
}

func bint(a bool) int {
	if a {
		return 1
	}
	return 0
}
