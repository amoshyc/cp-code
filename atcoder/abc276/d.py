import math


def solve():
    N = int(input())
    A = [int(x) for x in input().split()]
    g = A[0]
    for i in range(1, N):
        g = math.gcd(g, A[i])

    ans = 0
    for a in A:
        q = a // g
        if q == 1:
            continue

        while q != 1:
            if q % 2 == 0:
                q //= 2
                ans += 1
            elif q % 3 == 0:
                q //= 3
                ans += 1
            else:
                return -1
    return ans


print(solve())