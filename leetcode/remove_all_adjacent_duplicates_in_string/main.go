package main

func removeDuplicates(s string) string {
    bytes := []byte(s)
    i := 0
    for i < len(bytes) - 1 {
        if bytes[i] == bytes[i + 1] {
            bytes = append(bytes[:i], bytes[i + 2:]...)
            if i > 0 {
                i -= 1
            }
        } else {
            i += 1
        }
    }

    return string(bytes)
}
