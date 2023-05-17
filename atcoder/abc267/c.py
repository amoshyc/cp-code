import math

N, M = map(int, input().split())
A = [int(x) for x in input().split()]

W = [math.nan for _ in range(N)]
W[M - 1] = sum(A[:M])
for i in range(M, N):
    W[i] = W[i - 1] - A[i - M] + A[i]

dp = [-math.inf for _ in range(N)]
dp[M - 1] = sum((i + 1) * A[i] for i in range(M))
for i in range(M, N):
    dp[i] = dp[i - 1] - W[i - 1] + M * A[i]

print(max(dp))