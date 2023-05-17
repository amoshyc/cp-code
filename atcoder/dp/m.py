N, K = map(int, input().split())
M = 10**9 + 7
A = [int(x) for x in input().split()]

dp = [[0 for _ in range(K + 1)] for _ in range(N)]
for j in range(A[0] + 1):
    dp[0][j] = 1
for i in range(1, N):
    seg_sum = 0
    for j in range(K + 1):
        # dp[i][j] = sum(dp[i - 1][j - k] for k in range(min(A[i], j) + 1))
        # [j - min(A[i], j) - 1, j]
        seg_sum += dp[i - 1][j]
        if j - A[i] - 1 >= 0:
            seg_sum = seg_sum - dp[i - 1][j - A[i] - 1] + M
        seg_sum %= M
        
        dp[i][j] = seg_sum

print(dp[N - 1][K])