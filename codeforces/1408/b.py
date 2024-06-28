def ceil(a, b):
    return (a + b - 1) // b


def solve():
    N, K = map(int, input().split())
    A = set([int(x) for x in input().split()])

    if K == 1:
        return 1 if len(A) == 1 else -1

    # K + (m - 1) * (K - 1) >= len(A)
    # m >= (len(A) - K) / (K - 1) + 1
    if len(A) <= K:
        return 1
    return ceil(len(A) - K, K - 1) + 1


TC = int(input())
for _ in range(TC):
    print(solve())
