def solve():
    n = int(input())
    t = input()

    if n == 1:
        return 2 * 10 ** 10 if t == '1' else 10 ** 10

    if n == 2:
        if t == '00':
            return 0
        if t == '01':
            return 10 ** 10 - 1
        if t == '10':
            return 10 ** 10
        if t == '11':
            return 10 ** 10

    if t[:3] == '110':
        pass
    elif t[:2] == '10':
        t = '1' + t
    elif t[:1] == '0':
        t = '11' + t
    else:
        return 0

    if t[-3:] == '110':
        pass
    elif t[-2:] == '11':
        t = t + '0'
    elif t[-1:] == '1':
        t = t + '10'
    else:
        return 0

    if all(t[i : i + 3] == '110' for i in range(0, len(t), 3)):
        return 10 ** 10 - (len(t) // 3 - 1)
    return 0


print(solve())
