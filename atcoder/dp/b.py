N, K = map(int, input().split())
H = [int(x) for x in input().split()]

dp = [10 ** 10 for _ in range(N)]
dp[0] = 0
for i in range(N):
    for k in range(1, min(K + 1, N - i)):
        dp[i + k] = min(dp[i + k], dp[i] + abs(H[i + k] - H[i]))
print(dp[N - 1])

