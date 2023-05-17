N = int(input())
P = [float(x) for x in input().split()]

# dp[i][j] = Pr(number of head = j) after [0, i) coins are tossed
dp = [[0.0 for _ in range(N + 1)] for _ in range(N + 1)]
dp[0][0] = 1.0
for i in range(N):
    for j in range(N + 1):
        # coin i is tail
        dp[i + 1][j] += dp[i][j] * (1 - P[i])
        # coin i is head
        if j + 1 <= N:
            dp[i + 1][j + 1] += dp[i][j] * P[i]

ans = 0.0
for n_head in range(N + 1):
    if n_head > N - n_head:
        ans += dp[N][n_head]
print('{:.15f}'.format(ans))