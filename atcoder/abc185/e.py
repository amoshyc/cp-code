N, M = map(int, input().split())
A = [-1] + [int(x) for x in input().split()]
B = [-1] + [int(x) for x in input().split()]

dp = [[0 for _ in range(M + 1)] for _ in range(N + 1)]
for i in range(N + 1):
    dp[i][0] = i
for j in range(M + 1):
    dp[0][j] = j
for i in range(1, N + 1):
    for j in range(1, M + 1):
        if A[i] == B[j]:
            dp[i][j] = dp[i - 1][j - 1]
        else:
            dp[i][j] = min(dp[i - 1][j] + 1, dp[i][j - 1] + 1, dp[i - 1][j - 1] + 1)
print(dp[N][M])
