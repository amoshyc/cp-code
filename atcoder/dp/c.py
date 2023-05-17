N = int(input())

# dp[i, j] = the maximum value of day [0, i], and perform j on day i
dp = [[0, 0, 0] for _ in range(N + 1)]
dp[0] = [0, 0, 0]
for i in range(N):
    gain = [int(x) for x in input().split()]
    for j in range(3):  # today
        for k in range(3):  # tomorrow
            if j != k:
                dp[i + 1][k] = max(dp[i + 1][k], dp[i][j] + gain[j])
print(max(dp[-1]))
