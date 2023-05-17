N = int(input())
M = 10 ** 9 + 7
A = [list(map(int, input().split())) for _ in range(N)]

dp = [[0 for _ in range(1 << N)] for _ in range(N + 1)]
dp[0][0] = 1
for i in range(N):
    for s in range(1 << N):
        if dp[i][s] == 0:
            continue

        for j in range(N):
            if (s & (1 << j)) == 0 and A[i][j]:
                dp[i + 1][s | (1 << j)] += dp[i][s]
                dp[i + 1][s | (1 << j)] %= M

print(dp[N][(1 << N) - 1])
