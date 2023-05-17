N = int(input())
M = 10 ** 9 + 7
A = [[int(x) for x in input().split()] for _ in range(N)]

cost = [0 for _ in range(1 << N)]
for s in range(1 << N):
    for i in range(N):
        if s & (1 << i):
            for j in range(i + 1, N):
                if s & (1 << j):
                    cost[s] += A[i][j]

dp = [0 for _ in range(1 << N)]
for s in range(1 << N):
    t = s
    while t:
        dp[s] = max(dp[s], dp[s - t] + cost[t])
        t = (t - 1) & s

print(dp[(1 << N) - 1])
