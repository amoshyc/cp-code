import math


def solve():
    N, X, M = map(int, input().split())

    A = [X]
    vis = [-1] * M
    vis[X] = 0
    for i in range(1, M + 1):
        X = X ** 2 % M
        if vis[X] != -1:
            prefix = A[:vis[X]]
            cycle = A[vis[X]:]
            break
        A.append(X)
        vis[X] = i

    if N < len(prefix):
        return sum(prefix[:N])
    else:
        n_full_period = (N - len(prefix)) // len(cycle)
        suffix_length = N - len(prefix) - n_full_period * len(cycle)
        ans = 0
        ans += sum(prefix)
        ans += sum(cycle) * n_full_period
        ans += sum(cycle[:suffix_length])
        return ans

print(solve())