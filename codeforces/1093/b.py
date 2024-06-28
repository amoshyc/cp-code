from collections import defaultdict

def solve():
    s = input()
    if s != s[::-1]:
        return s

    s = list(s)

    for idx in range(1, len(s) - 1):
        if s[idx] != s[0]:
            s[idx], s[0] = s[0], s[idx]
            return ''.join(s)

    return -1

T = int(input())
for _ in range(T):
    print(solve())
