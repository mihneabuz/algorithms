package main

func maximum69Number (num int) int {
    copy, six := num, -1
    for i := 0; copy > 0; i += 1 {
        if copy % 10 == 6 {
            six = i
        }
        copy /= 10
    }

    if six != -1 {
        num += 3 * IntPow(10, six)
    }

    return num
}

func IntPow(x, exp int) int {
    if exp == 0 {
        return 1
    }

    r := IntPow(x, exp / 2)
    if exp % 2 == 0 {
        return r * r
    } else {
        return r * r * x
    }
}
