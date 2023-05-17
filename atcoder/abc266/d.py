T = 10 ** 5 + 1

N = int(input())
E = [[0, 0, 0, 0, 0] for _ in range(T)]
for _ in range(N):
    t, x, a = map(int, input().split())
    E[t][x] = a

# dp[i, j] = 在 i 秒，人在 pit j 時，所能得到的最大價值
dp = [[0, -10 ** 8, -10 ** 8, -10 ** 8, -10 ** 8] for _ in range(T)]

for i in range(1, T):
    for j in range(5):
        dp[i][j] = dp[i - 1][j]
        if j >= 1:
            dp[i][j] = max(dp[i][j], dp[i - 1][j - 1])
        if j + 1 < 5:
            dp[i][j] = max(dp[i][j], dp[i - 1][j + 1])
        if dp[i][j] >= 0:
            dp[i][j] += E[i][j]

print(max(dp[-1]))