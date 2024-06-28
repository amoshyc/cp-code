from collections import Counter

def solve():
    N = int(input())
    A = list(map(int, input().split()))
    C = sorted(Counter(A).values(), reverse=True)

    def check(m):
        for i, c in enumerate(C):
            if (c - 1) * (m + 1) + i >= N:
                return False
        return True

    lb, ub = -1, N
    while ub - lb > 1:
        m = (lb + ub) // 2
        if check(m):
            lb = m
        else:
            ub = m
    return lb

TC = int(input())
for _ in range(TC):
    print(solve())