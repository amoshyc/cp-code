from functools import lru_cache

N, X = map(int, input().split())
A = [int(x) for x in input().split()]


@lru_cache(maxsize=None)
def f(x, i):
    if i == N - 1:
        return 1

    r = x % A[i + 1]
    if r == 0:
        return f(x, i + 1)
    else:
        return f(x - r, i + 1) + f(x - r + A[i + 1], i + 1)


print(f(X, 0))
