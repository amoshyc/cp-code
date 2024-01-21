N, K = map(int, input().split())
mod = 10**9 + 7
A = [int(x) for x in input().split()]

dp = [[0 for _ in range(K + 1)] for _ in range(N)]
for j in range(A[0] + 1):
    dp[0][j] = 1
for i in range(1, N):
    window_sum = 0
    for j in range(K + 1):
        window_sum += dp[i - 1][j]
        if j - A[i] - 1 >= 0:
            window_sum -= dp[i - 1][j - A[i] - 1]
        window_sum %= mod

        dp[i][j] = window_sum

print(dp[N - 1][K])